use tui::{
    layout::Rect,
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget},
};

use crate::{config::Styles, TorrentCmd};

enum TorrentDialogField {
    Url,
    Path,
}

pub(crate) struct TorrentDialogState {
    selected_field: TorrentDialogField,
    path: String,
    url: String,
}
impl Default for TorrentDialogState {
    fn default() -> Self {
        Self {
            selected_field: TorrentDialogField::Path,
            path: "".to_string(),
            url: "".to_string(),
        }
    }
}
impl TorrentDialogState {
    pub(crate) fn input(&mut self) -> &mut String {
        match self.selected_field {
            TorrentDialogField::Url => &mut self.url,
            TorrentDialogField::Path => &mut self.path,
        }
    }
    pub(crate) fn next(&mut self) {
        self.selected_field = match self.selected_field {
            TorrentDialogField::Url => TorrentDialogField::Path,
            TorrentDialogField::Path => TorrentDialogField::Url,
        }
    }
    pub(crate) fn command(&self) -> TorrentCmd {
        // download dir, filename, metainfo, start_paused
        TorrentCmd::AddTorrent(Some(self.path.clone()), Some(self.url.clone()), None, true)
    }
}
pub(crate) struct TorrentDialog<'a> {
    styles: &'a Styles,
}
impl<'a> TorrentDialog<'a> {
    pub(crate) fn new(styles: &'a Styles) -> Self {
        Self { styles }
    }
}
impl<'a> StatefulWidget for TorrentDialog<'a> {
    type State = TorrentDialogState;

    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {
        let styles = &self.styles;
        let lines = vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![
                Span::styled(" Add torrent: ", styles.text),
                Span::raw(state.path.to_string()),
            ]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![
                Span::styled(" Enter torrent URL or path: ", styles.text),
                Span::raw(state.url.to_string()),
            ]),
        ];
        let message = Paragraph::new(lines).block(
            Block::default()
                .title("Add Torrent")
                .borders(Borders::ALL)
                .border_style(styles.text),
        );
        Clear {}.render(area, buf);
        message.render(area, buf);
    }
}
