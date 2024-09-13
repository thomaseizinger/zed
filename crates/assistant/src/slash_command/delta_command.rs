use anyhow::{anyhow, Result};
use assistant_slash_command::{
    AfterCompletion, ArgumentCompletion, SlashCommand, SlashCommandOutput,
    SlashCommandOutputSection,
};
use gpui::{AppContext, Task, View, WeakView, WindowContext};
use language::{Anchor, BufferSnapshot, LspAdapterDelegate};
use std::sync::{atomic::AtomicBool, Arc};
use workspace::Workspace;

pub(crate) struct DeltaSlashCommand;

impl DeltaSlashCommand {}

impl SlashCommand for DeltaSlashCommand {
    fn name(&self) -> String {
        "delta".into()
    }

    fn description(&self) -> String {
        "re-insert files that have changed".into()
    }

    fn menu_text(&self) -> String {
        todo!()
    }

    fn requires_argument(&self) -> bool {
        todo!()
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        cancellation_flag: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        todo!()
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: Vec<SlashCommandOutputSection<language::Anchor>>,
        _context_buffer: BufferSnapshot,
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        todo!()
    }
}
