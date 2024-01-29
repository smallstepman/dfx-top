use ratatui::prelude::*;

///  +-----------------------------+------------+
///  |      NETWORK                |  IDENTITY  |
///  |                             |            |
///  +--------------------+--------+------------+
///  |      CANISTERS     |                     |
///  |                    |      LOGS           |
///  +--------------------|                     |
///  |  CANISTER INFO     |                     |
///  |                    |                     |
///  +--------------------+---------------------+
pub fn get_layout(frame_size: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(frame_size);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(chunks[1]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(82), Constraint::Percentage(18)])
        .split(bottom_chunks[0]);

    return (
        top_chunks[0],    // networks
        top_chunks[1],    // identity
        left_chunks[0],   // canister ids
        left_chunks[1],   // canister info
        bottom_chunks[1], // logs
    );
}
