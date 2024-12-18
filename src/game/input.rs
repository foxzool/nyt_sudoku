use crate::game::UpdateCell;
use crate::game::cell_state::{CellValue, FixedCell};
use crate::game::position::CellPosition;
use crate::game::{CleanCell, NewValueChecker, SelectedCell};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::{Commands, Entity, EventReader, KeyCode, Single, With};
use sudoku::board::{CellState, Digit};

pub(crate) fn keyboard_input(
    mut commands: Commands,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut selected_cell: Single<
        Entity, With<SelectedCell>,
    >,
) {

    for event in keyboard_input_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        if event.key_code == KeyCode::Delete {
            commands.trigger_targets(CleanCell, vec![*selected_cell]);
        }

        let num = match event.key_code {
            KeyCode::Digit0 | KeyCode::Numpad0 => Some(0),
            KeyCode::Digit1 | KeyCode::Numpad1 => Some(1),
            KeyCode::Digit2 | KeyCode::Numpad2 => Some(2),
            KeyCode::Digit3 | KeyCode::Numpad3 => Some(3),
            KeyCode::Digit4 | KeyCode::Numpad4 => Some(4),
            KeyCode::Digit5 | KeyCode::Numpad5 => Some(5),
            KeyCode::Digit6 | KeyCode::Numpad6 => Some(6),
            KeyCode::Digit7 | KeyCode::Numpad7 => Some(7),
            KeyCode::Digit8 | KeyCode::Numpad8 => Some(8),
            KeyCode::Digit9 | KeyCode::Numpad9 => Some(9),
            _ => None,
        };

        if let Some(num) = num {
            commands.trigger_targets(CleanCell, vec![*selected_cell]);
            commands.trigger_targets(UpdateCell(CellState::Digit(Digit::new(num))), vec![*selected_cell]);
        }
    }
}
