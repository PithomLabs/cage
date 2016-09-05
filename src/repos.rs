//! APIs for working with the git repositories associated with a `Project`.

use std::collections::BTreeMap;
use std::collections::btree_map;
use std::path::{Path, PathBuf};

use ext::service::ServiceExt;
use git_url::GitUrl;
use pod::Pod;
use util::Error;

/// All the git repositories associated with a project.
#[derive(Debug)]
pub struct Repos {
    /// Our repositories, indexed by their local alias.
    repos: BTreeMap<String, Repo>,
}

impl Repos {
    /// Create a collection of repositories based on a list of pods.
    #[doc(hidden)]
    pub fn new(pods: &[Pod]) -> Result<Repos, Error> {
        let mut repos: BTreeMap<String, Repo> = BTreeMap::new();
        for pod in pods {
            for file in pod.all_files() {
                for (_name, service) in &file.services {
                    if let Some(git_url) = try!(service.git_url()) {
                        // Figure out what alias we want to use.  The final
                        // `unwrap` should be safe because we know the input
                        // was UTF-8.
                        //
                        // TODO LOW: We may need to unescape this.
                        let url_path =
                            Path::new(try!(git_url.to_url()).path()).to_owned();
                        let alias = try!(url_path.file_stem().ok_or_else(|| {
                            err!("Can't get repo name from {}", &git_url)
                        })).to_str().unwrap().to_owned();

                        // Build our repository.
                        let repo = Repo { alias: alias, git_url: git_url };

                        // Insert our repository our map, checking for alias
                        // clashes.
                        match repos.entry(repo.alias.clone()) {
                            btree_map::Entry::Vacant(vacant) => {
                                vacant.insert(repo);
                            }
                            btree_map::Entry::Occupied(occupied) => {
                                if &repo.alias != &occupied.get().alias {
                                    return Err(err!("{} and {} would both alias to {}",
                                                    &occupied.get().git_url,
                                                    &repo.git_url,
                                                    &repo.alias));
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(Repos { repos: repos })
    }

    /// Iterate over all repositories associated with this project.
    pub fn iter(&self) -> Iter {
        Iter { iter: self.repos.iter() }
    }

    /// Look up a repository using the short-form local alias.
    pub fn find_by_alias(&self, alias: &str) -> Option<&Repo> {
        self.repos.get(alias)
    }
}

/// An iterator over all repositories associated with this project.
#[derive(Clone)]
pub struct Iter<'a> {
    // We wrap this in our own struct to make the underlying type opaque.
    iter: btree_map::Iter<'a, String, Repo>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Repo;

    fn next(&mut self) -> Option<&'a Repo> {
        self.iter.next().map(|(_alias, repo)| repo)
    }
}

/// A single repository.
#[derive(Debug)]
pub struct Repo {
    alias: String,
    git_url: GitUrl,
}

impl Repo {
    /// A short local name for this git repository, suitable for use as
    /// a directory name or command-line argument.
    pub fn alias(&self) -> &str {
        &self.alias
    }

    /// The remote git URL from which we can clone this repository.
    pub fn git_url(&self) -> &GitUrl {
        &self.git_url
    }

    /// The path to which we would check out this repository, relative to
    /// `Project::output_dir`.
    pub fn rel_path(&self) -> PathBuf {
        Path::new("src").join(self.alias())
    }
}

#[test]
fn are_loaded_with_projects() {
    //use command_runner::TestCommandRunner;
    use project::Project;

    let proj = Project::from_example("hello").unwrap();
    let repos = proj.repos();
    assert_eq!(repos.iter().count(), 2);
    let hello = repos.find_by_alias("dockercloud-hello-world")
        .expect("repos should include dockercloud-hello-world");
    assert_eq!(hello.alias(), "dockercloud-hello-world");
    assert_eq!(hello.git_url().as_ref() as &str,
               "https://github.com/docker/dockercloud-hello-world.git");
    assert_eq!(hello.rel_path(), Path::new("src/dockercloud-hello-world"));

    //let runner = TestCommandRunner::new();
    // TODO: Implement clone.
}
