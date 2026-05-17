use mdbook_preprocessor::{
    book::{Book, BookItem},
    errors::Result,
    Preprocessor, PreprocessorContext,
};
use pulldown_cmark::{Event, Tag, TagEnd};
use pulldown_cmark_to_cmark::cmark;

use crate::{CompositeError, Mode};

pub struct TrplNote;

impl TrplNote {
    pub fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer == "html" || renderer == "markdown" || renderer == "test")
    }
}

impl Preprocessor for TrplNote {
    fn name(&self) -> &str {
        "trpl-note"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mode = Mode::from_context(ctx, self.name())?;

        let mut errors = vec![];
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                match rewrite_notes(&chapter.content, mode) {
                    Ok(rewritten) => chapter.content = rewritten,
                    Err(reason) => errors.push(reason),
                }
            }
        });

        if errors.is_empty() {
            Ok(book)
        } else {
            Err(CompositeError(errors).into())
        }
    }
}

fn rewrite_notes(src: &str, mode: Mode) -> anyhow::Result<String> {
    if mode == Mode::Default {
        return Ok(src.into());
    }

    #[derive(Default)]
    struct State<'e> {
        in_note: bool,
        events: Vec<Event<'e>>,
    }

    let final_state: State = crate::parser(src).try_fold(
        State::default(),
        |mut state, event| -> anyhow::Result<State> {
            if state.in_note {
                match event {
                    Event::Start(Tag::Emphasis | Tag::Strong | Tag::Strikethrough)
                    | Event::End(TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough)
                    | Event::InlineHtml(_) => { /* skip */ }
                    Event::Code(code) => state.events.push(Event::Text(code)),
                    Event::End(TagEnd::BlockQuote) => {
                        state.in_note = false;
                        state.events.push(event);
                    }
                    _ => state.events.push(event),
                }
            } else if matches!(event, Event::Start(Tag::BlockQuote)) {
                state.events.push(event);
                state.in_note = true;
            } else {
                state.events.push(event);
            }
            Ok(state)
        },
    )?;

    if final_state.in_note {
        return Err(anyhow::anyhow!("Unclosed note"));
    }

    let mut rewritten = String::new();
    cmark(final_state.events.into_iter(), &mut rewritten)?;
    Ok(rewritten)
}

#[cfg(test)]
mod tests;
