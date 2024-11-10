mod actor;

use kameo;
use crate::actor::{CrossTerm};

#[tokio::main]
async fn main() {

    // all options either have all necessary args passed via cli, or drop user into TUI
    // if new TODO
    // if init (alias of new) TODO
    // if help TODO
    // if open (uri or path to file, will import from/export to git/fossil if that is the ) TODO
    // if local-conf (global configuration values to local user, like git) TODO
    // if watch (enter interactive session) TODO
    //
    // UCM commands:
    // Note: This seems like a LOT of commands, should try to simplify a lot.
    // if add (Adds all the definitions from the most recently typechecked file to the codebase.) TODO
    // if alias.term (adds a second name for a term, not a rename) TODO
    // if alias.type (adds a second name for a type, not a rename) TODO
    // if auth.login (*not* supported, login will be handled when open from uri fails)
    // if create.author (alias of local-conf) TODO
    // if compile (build the source! will work for standard build systems like make, for
    //             non-supported build systems, the project must provide a how-to-build (or alias)
    //             in the project's README.md (or alias)) TODO
    // if copy.patch (Use copy.patch foo bar to copy a patch from foo to bar) TODO
    // if debug.clear-cache (Used for clearing the UCM cache of previous test or watch expression runs.) TODO
    // if delete (Deletes the given term or type name from the codebase.) TODO
    // if delete.namespace (deletes the namespace and the terms it contains from the codebase. ) TODO
    // if delete.namespace.force (Removes the namespace even if the terms within that namespace are used in the codebase.) TODO
    // if delete.patch (Removes the patch in the given namespace) TODO
    // if dependancies (Lists the dependencies of the specified definition. Accepts a term name or hash.) TODO
    // if dependants (The dependents command lists all the terms that make use of the given term or type in their implementation.) TODO
    // if display (Displays a rendered version of the given term to the console.) TODO
    // if display.to (*not* supported, just pipe to file)
    // if docs (prints the docs for the given term) TODO
    // if edit (edit prepends the definition of the given argument(s) to the top of the most recently saved file.) TODO
    // if find (find foo bar baz searches the current namespace tree for the given argument(s), excluding the lib directory.) TODO
    // if find.all (*not* supported. barck fina is equivalent to UCM find.all)
    // if find.verbose (is find, but also prints the hashs. Alias of find --verbose) TODO
    // if fork (Creates a copy of the given source namespace at the a new destination.) TODO
    // if gist (Pushes the contents of the namespace in which it is called to the given remote repository.) TODO
    // if history (command displays the history of changes) TODO
    // if log (alias of history) TODO
    // if reflog (alias of history) TODO
    // if global.reflog (alias of history) TODO
    // if project.reflog (alias of history) TODO
    // if io.test (*not* supported)
    // if ls (displays the terms, types, and sub-namespaces in the given namespace.) TODO
    // if load (*not* supported)
    // if merge (merges the source namespace into the destination) TODO
    // if merge.preview (*not* supported, merge will preview unless specified to force)
    // if move.term (renames an existing term) TODO
    // if move.type (renames an existing type) TODO
    // if patch (Rewrites any definitions that depend on definitions with type-preserving edits to use the updated versions of these dependencies.) TODO
    // if pull (*not* supported, Barck syncs local and remote automatically)
    // if pull-request.load (*not* supported, Barck syncs local and remote.)
    // if push (*not* supported. Barck syncs local and remote automatically)
    // if reset (command will undo the add that happened in the reflog, targeted undo at changes
    //           prior to most recent. If changing most recent, use undo) TODO
    // if run (either run automatically with supported build systems, or print provided
    //         instructions from README.md or alias, similar to compile) TODO
    // if run.compiled (*not* supported, just use run)
    // if todo (TODO)
    // if ui (*not* supproted, we natively drop to TUI if command isn;t complete)
    // if undo (Use undo to revert the most recent change to the codebase.) TODO
    // if update (Adds everything in the most recently typechecked file to the namespace) TODO
    // if upgrade (Upgrades the given dependency to a newer version.) TODO
    // if upgrade.commit (*not* supported, just make upgrade good)
    // if version (print all relevant version information, barck and repo info if present) TODO
    // if view (displays the source code of the given Unison term or type.) TODO

    let crossterm = kameo::spawn(CrossTerm::new());
    crossterm.wait_for_stop().await;
    println!("Goodbye.");
}
