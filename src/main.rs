#[macro_use]
extern crate nickel;
extern crate atelier;

use nickel::Nickel;
use nickel::router::http_router::HttpRouter;
use nickel::JsonBody;
use atelier::file_set::{ FileSet, FileData };
use atelier::repository_locator::{ self, RepositoryState };

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    //curl 'http://localhost:6767/kanvaz' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "files": [{ "name":"style.css", "content": "button: { color: red; }"}] }'
    router.post("/kanvaz", middleware! { |request, response|
        let file_set = request.json_as::<FileSet>().unwrap();
        let repository = repository_locator::get_repository_handle(RepositoryState::NonExisting);
        repository.add_files_and_commit(file_set.files, "SAVEPOINT");
        format!("{:?}", repository)
    });

    //curl 'http://localhost:6767/kanvaz/<PUT-REPOSITORY-ID-HERE>' -X PUT -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "files": [{ "name":"style.css", "content": "button: { color: green; }"}] }'
    router.put("/kanvaz/:id", middleware! { |request, response|
        let file_set = request.json_as::<FileSet>().unwrap();

        match request.param("id") {
            Some(id) => {
                let repository = repository_locator::get_repository_handle(RepositoryState::Existing(id));
                repository.add_files_and_commit(file_set.files, "SAVEPOINT");
                format!("{:?}", repository)
            }
            _ => format!("no id given")
        }
    });

    //curl 'http://localhost:6767/kanvaz/new'
    router.get("/kanvaz/new", middleware! { |_, response|
        let file_set = FileSet {
            files: vec!(
                        FileData { name: "index.html".to_string(), content: "<html>
                            <head>
                                <title>New Kanvaz</title>
                            </head>
                            <body>
                            </body>
                        </html>".to_string() },
                        FileData { name: "style.css".to_string(), content: "/*put your styles here*/".to_string() },
                        FileData { name: "app.js".to_string(), content: "/*put your js here*/".to_string() },
                    )
        };
        format!("{}", file_set.to_pretty_json())
    });

    //curl 'http://localhost:6767/kanvaz/<PUT-REPOSITORY-ID-HERE>'
    router.get("/kanvaz/:id", middleware! { |request, response|
        match request.param("id") {
            Some(id) => {
                let repository = repository_locator::get_repository_handle(RepositoryState::Existing(id));
                let file_set = FileSet { files: repository.read_all_files() };
                format!("{}", file_set.to_pretty_json())
            }
            _ => format!("no id given")
        }
    });

    server.utilize(router);

    server.listen("127.0.0.1:6767");
}
