use colored::Colorize;

pub(crate) fn git() -> Option<String> {
    let repo: Repository = git2::Repository::discover(".").ok()?.into();

    let mut output = match repo.head() {
        Some(head) => match head.shorthand() {
            Some(branch) => String::from(branch),
            None => {
                return Some(colorize("unknown branch"));
            }
        },
        None => {
            return Some(colorize("no head"));
        }
    };

    if let Some(ref hash) = repo.hash() {
        output.push('@');
        output.push_str(hash);
    }

    // if let Some(ref status) = repo.status() {
    //     output.push(' ');
    //     output.push_str(status);
    // }

    // if let Some(ref state) = repo.state() {
    //     output.push(' ');
    //     output.push_str(state);
    // }

    Some(colorize(output))
}

fn colorize<S>(s: S) -> String
where
    S: AsRef<str>,
{
    format!(" {} ", s.as_ref())
        .bold()
        .color("black")
        .on_color("white")
        .to_string()
}

struct Repository(git2::Repository);

impl From<git2::Repository> for Repository {
    fn from(repo: git2::Repository) -> Self {
        Self(repo)
    }
}

impl Repository {
    fn hash(&self) -> Option<String> {
        let s = self.head()?.target()?.to_string().chars().take(7).collect();
        Some(s)
    }

    fn head<'a>(&'a self) -> Option<git2::Reference<'a>> {
        self.0.head().ok()
    }

    #[allow(dead_code)]
    fn state(&self) -> Option<&'static str> {
        use git2::RepositoryState::*;
        match self.0.state() {
            ApplyMailbox => Some("apply-mailbox"),
            ApplyMailboxOrRebase => Some("apply-mailbox"),
            Bisect => Some("bisect"),
            CherryPick => Some("cherry-pick"),
            CherryPickSequence => Some("cherry-pick"),
            Clean => None,
            Merge => Some("merge"),
            Rebase => Some("rebase"),
            RebaseInteractive => Some("rebase"),
            RebaseMerge => Some("rebase-merge"),
            Revert => Some("revert"),
            RevertSequence => Some("revert"),
        }
    }

    #[allow(dead_code)]
    fn status(&self) -> Option<String> {
        let mut output = String::new();
        let mut seen = Vec::new();

        for status in self.0.statuses(None).unwrap().iter() {
            if let Some(status) = status.index_to_workdir() {
                let delta = status.status();

                if seen.contains(&delta) {
                    continue;
                }
                seen.push(delta);

                use git2::Delta::*;
                match delta {
                    Added => output.push('+'),
                    Copied | Ignored | Unmodified | Unreadable => {}
                    Conflicted => output.push('#'),
                    Deleted => output.push('-'),
                    Modified | Renamed | Typechange => output.push('*'),
                    Untracked => output.push('?'),
                }
            }
        }

        if output.is_empty() {
            None
        } else {
            Some(output)
        }
    }
}
