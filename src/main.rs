mod actor;

use kameo;
use crate::actor::{CrossTerm};

use clap::{Subcommand, Parser};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand, Clone)]
enum SubCommands {
    /// TODO Open a project from local file, remote barck file, git repo, fossil repo, or if none specified, select from previous selections.
    Open,
    #[command(alias("configure"))]
    #[command(alias("create"))]
    /// TODO Local configuration for user preferances.
    UserConf,
    /// TODO Local configuration for interfacing with remote project
    ProjectConf,
    #[command(alias("a"))]
    /// Adds everything in the most recently typechecked file to the namespace.
    /// Called Update in UCM. Barck does not have the UCM equivalent to add.
    Add,
    /// TODO add a second name for given name
    Alias,
    #[command(alias("c"))]
    #[command(alias("b"))]
    #[command(alias("compile"))]
    /// TODO Compile local project.
    /// Will automatically build for supported build systems,
    /// Unsupported build systems must link to README to output build instructions
    Build,
    /// TODO Copy patch patch from one term to another.
    CopyPatch,
    /// TODO Clear build cache.
    /// Will automatically build for supported build systems,
    /// Unsupported build systems must link to README to output build instructions
    ClearCache,
    /// TODO Deletes the given term or type name from the codebase.
    Delete,
    /// TODO Lists the dependencies of the specified definition. Accepts a term name or hash.
    Dependencies,
    /// TODO List reverse dependencies
    Dependents,
    /// TODO A rendered version of the given term to the console
    Display,
    /// TODO Prints the docs for the given term
    Docs,
    /// TODO Prepends the definition of the given argument(s) to the top of the most recently saved file.
    Edit,
    /// Search for a term.
    Find,
    /// TODO Creates a copy of the given source namespace at the a new destination.
    Fork,
    /// TODO Pushes the contents of the namespace in which it is called to the given remote repository.
    Gist,
    #[command(alias("log"))]
    #[command(alias("reflog"))]
    /// TODO history displays as list of changes.
    History,
    #[command(alias("ls"))]
    /// TODO Displays the terms, types, and sub-namespaces in the given namespace.
    List,
    /// TODO Merges the source namespace into the destination.
    /// will start remote approval process if necessary.
    Merge,
    #[command(alias("rename"))]
    #[command(alias("mv"))]
    /// TODO renames an existing term.
    Move,
    #[command(alias("init"))]
    /// TODO Create a new Barck project
    New {
        name: Option<String>,
    },
    /// TODO Rewrites any definitions that depend on definitions with type-preserving edits to use the updated versions of these dependencies.
    Patch,
    #[command(alias("r"))]
    /// TODO Run project with supported build systems, or print provided
    /// instructions from README.md or alias, similar to build.
    Run,
    /// TODO
    Todo,
    /// TODO Undo last change to project, or specified change in history.
    #[command(alias("u"))]
    Undo,
    /// TODO Upgrades the given dependency to a newer version for supported build systems.
    Upgrade,
    /// TODO Print all relevant version information, barck and repo info if present.
    Version,
    /// TODO Displays the source code of the given Unison term or type.
    View,
}

#[tokio::main]
async fn main() {
    // TODO remove Option<supervisor> once all commands are implemented
    if let Some(s) = match Cli::parse().command {
        SubCommands::New{name: None} => Some(kameo::spawn(TUI::new(SubCommands::New{name: None}))),
        SubCommands::New{name: Some(n)} => Some(kameo::spawn(New::new(n))),
        _ => println!("Not supported yet."),
    } {
        s.wait_for_stop().await;
    }

    println!("Goodbye.");

    //let crossterm = kameo::spawn(CrossTerm::new());
    //crossterm.wait_for_stop().await;
}
