use anyhow::Result;
use code_blocks_server::get_subtrees;
use code_blocks_server::move_block;
use code_blocks_server::GetSubtreesArgs;
use code_blocks_server::MoveBlockArgs;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "status", content = "result")]
pub enum JsonResult<T> {
    Ok(T),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method", content = "params")]
pub enum MethodCall {
    GetSubtrees(GetSubtreesArgs),
    MoveBlock(MoveBlockArgs),
}

fn main() {
    for line in std::io::stdin().lines() {
        let Ok(line) = line else { continue; };

        let response = handle_request(&line);

        let Ok(response) = serde_json::to_string(&response) else {
            eprintln!("Failed to serialize response");
            continue;
        };

        println!("{}", response);
    }
}

fn handle_request(request: &str) -> JsonResult<Value> {
    match handle_line(request) {
        Ok(ok) => JsonResult::Ok(ok),
        Err(err) => JsonResult::Error(err.to_string()),
    }
}

fn handle_line(line: &str) -> Result<Value> {
    match serde_json::from_str::<MethodCall>(line)? {
        MethodCall::GetSubtrees(args) => Ok(serde_json::to_value(&get_subtrees(args)?)?),
        MethodCall::MoveBlock(args) => Ok(serde_json::to_value(&move_block(args)?)?),
    }
}

#[cfg(test)]
mod tests {
    use code_blocks_server::{BlockLocation, SupportedLanguage};

    use super::*;

    #[test]
    fn show_get_subtrees_request() {
        insta::assert_json_snapshot!(
            MethodCall::GetSubtrees(GetSubtreesArgs {
                queries: vec!["(function_item) @ident".to_string()],
                text: "fn main() {}\nfn foo() {}".to_string(),
                language: SupportedLanguage::Rust,
            }),
            @r###"
        {
          "method": "getSubtrees",
          "params": {
            "queries": [
              "(function_item) @ident"
            ],
            "text": "fn main() {}\nfn foo() {}",
            "language": "rust"
          }
        }
        "###
        );
    }

    #[test]
    fn show_get_subtrees_response() {
        insta::assert_json_snapshot!(handle_request(
            r#"
        {
          "method": "getSubtrees",
          "params": {
            "queries": [
              "(function_item) @ident"
            ],
            "text": "fn main() {}\nfn foo() {}",
            "language": "rust"
          }
        }
        "#
        ),
        @r###"
        {
          "status": "ok",
          "result": [
            {
              "block": {
                "endByte": 12,
                "endCol": 12,
                "endRow": 0,
                "startByte": 0,
                "startCol": 0,
                "startRow": 0
              },
              "children": []
            },
            {
              "block": {
                "endByte": 24,
                "endCol": 11,
                "endRow": 1,
                "startByte": 13,
                "startCol": 0,
                "startRow": 1
              },
              "children": []
            }
          ]
        }
        "###);

        insta::assert_json_snapshot!(handle_request(
            r#"
        {
          "method": "badMethod",
          "params": {
            "queries": [
              "(function_item) @ident"
            ],
            "text": "fn main() {}\nfn foo() {}",
            "language": "rust"
          }
        }
        "#
        ),
        @r###"
        {
          "status": "error",
          "result": "unknown variant `badMethod`, expected `getSubtrees` or `moveBlock` at line 3 column 31"
        }
        "###);

        insta::assert_json_snapshot!(handle_request(
            r#"
        {
          "method": "getSubtrees",
          "params": {
            "queries": [
              "bad query"
            ],
            "text": "fn main() {}\nfn foo() {}",
            "language": "rust"
          }
        }
        "#
        ),
        @r###"
        {
          "status": "error",
          "result": "Query error at 1:1. Invalid syntax:\nbad query\n^"
        }
        "###);

        insta::assert_json_snapshot!(handle_request(
            r#"
        {
          "method": "getSubtrees",
          "params": {
            "queries": [
              "(function_item) @ident"
            ],
            "text": "fn main() {}\nfn foo() {}",
            "language": "bad language"
          }
        }
        "#
        ),
        @r###"
        {
          "status": "error",
          "result": "unknown variant `bad language`, expected one of `rust`, `typescript`, `tsx`, `svelte` at line 9 column 38"
        }
        "###);
    }

    #[test]
    fn show_move_block_request() {
        insta::assert_json_snapshot!(
            MethodCall::MoveBlock(MoveBlockArgs {
                queries: vec!["(function_item) @ident".to_string()],
                text: "fn main() {}\nfn foo() {}".to_string(),
                language: SupportedLanguage::Rust,
                src_block: BlockLocation::default(),
                dst_block: BlockLocation::default()
            }),
            @r###"
        {
          "method": "moveBlock",
          "params": {
            "queries": [
              "(function_item) @ident"
            ],
            "text": "fn main() {}\nfn foo() {}",
            "language": "rust",
            "srcBlock": {
              "startByte": 0,
              "endByte": 0,
              "startRow": 0,
              "startCol": 0,
              "endRow": 0,
              "endCol": 0
            },
            "dstBlock": {
              "startByte": 0,
              "endByte": 0,
              "startRow": 0,
              "startCol": 0,
              "endRow": 0,
              "endCol": 0
            }
          }
        }
        "###
        );
    }

    #[test]
    fn show_move_block_response() {
        insta::assert_json_snapshot!(handle_request(
                r###"
            {
            "method": "moveBlock",
            "params": {
                "queries": [
                "(function_item) @ident"
                ],
                "text": "fn main() {}\nfn foo() {}",
                "language": "rust",
                "srcBlock": {
                    "startByte": 0,
                    "endByte": 12,
                    "startRow": 0,
                    "startCol": 0,
                    "endRow": 0,
                    "endCol": 12
                },
                "dstBlock": {
                    "startByte": 13,
                    "endByte": 24,
                    "startRow": 1,
                    "startCol": 0,
                    "endRow": 1,
                    "endCol": 11
                }
            }
            }
            "###),
            @r###"
        {
          "status": "ok",
          "result": "fn foo() {}\nfn main() {}"
        }
        "###
        );

        insta::assert_json_snapshot!(handle_request(
                r###"
            {
            "method": "badMethod",
            "params": {
                "queries": [
                "(function_item) @ident"
                ],
                "text": "fn main() {}\nfn foo() {}",
                "language": "rust",
                "srcBlock": {
                    "startByte": 0,
                    "endByte": 12,
                    "startRow": 0,
                    "startCol": 0,
                    "endRow": 0,
                    "endCol": 12
                },
                "dstBlock": {
                    "startByte": 13,
                    "endByte": 24,
                    "startRow": 1,
                    "startCol": 0,
                    "endRow": 1,
                    "endCol": 11
                }
            }
            }
            "###),
            @r###"
        {
          "status": "error",
          "result": "unknown variant `badMethod`, expected `getSubtrees` or `moveBlock` at line 3 column 33"
        }
        "###
        );

        insta::assert_json_snapshot!(handle_request(
                r###"
            {
            "method": "moveBlock",
            "params": {
                "queries": [
                "bad query"
                ],
                "text": "fn main() {}\nfn foo() {}",
                "language": "rust",
                "srcBlock": {
                    "startByte": 0,
                    "endByte": 12,
                    "startRow": 0,
                    "startCol": 0,
                    "endRow": 0,
                    "endCol": 12
                },
                "dstBlock": {
                    "startByte": 13,
                    "endByte": 24,
                    "startRow": 1,
                    "startCol": 0,
                    "endRow": 1,
                    "endCol": 11
                }
            }
            }
            "###),
            @r###"
        {
          "status": "error",
          "result": "Query error at 1:1. Invalid syntax:\nbad query\n^"
        }
        "###
        );

        insta::assert_json_snapshot!(handle_request(
                r###"
            {
            "method": "moveBlock",
            "params": {
                "queries": [
                "(function_item) @ident"
                ],
                "text": "bad text",
                "language": "rust",
                "srcBlock": {
                    "startByte": 0,
                    "endByte": 12,
                    "startRow": 0,
                    "startCol": 0,
                    "endRow": 0,
                    "endCol": 12
                },
                "dstBlock": {
                    "startByte": 13,
                    "endByte": 24,
                    "startRow": 1,
                    "startCol": 0,
                    "endRow": 1,
                    "endCol": 11
                }
            }
            }
            "###),
            @r###"
        {
          "status": "error",
          "result": "Failed to find src item"
        }
        "###
        );

        insta::assert_json_snapshot!(handle_request(
                r###"
            {
            "method": "moveBlock",
            "params": {
                "queries": [
                "(function_item) @ident"
                ],
                "text": "fn main() {}\nfn foo() {}",
                "language": "bad language",
                "srcBlock": {
                    "startByte": 0,
                    "endByte": 12,
                    "startRow": 0,
                    "startCol": 0,
                    "endRow": 0,
                    "endCol": 12
                },
                "dstBlock": {
                    "startByte": 13,
                    "endByte": 24,
                    "startRow": 1,
                    "startCol": 0,
                    "endRow": 1,
                    "endCol": 11
                }
            }
            }
            "###),
            @r###"
        {
          "status": "error",
          "result": "unknown variant `bad language`, expected one of `rust`, `typescript`, `tsx`, `svelte` at line 9 column 42"
        }
        "###
        );

        insta::assert_json_snapshot!(handle_request(
                r###"
            {
            "method": "moveBlock",
            "params": {
                "queries": [
                "(function_item) @ident"
                ],
                "text": "fn main() {}\nfn foo() {}",
                "language": "rust",
                "srcBlock": {
                    "startByte": 999,
                    "endByte": 999,
                    "startRow": 999,
                    "startCol": 999,
                    "endRow": 999,
                    "endCol": 999
                },
                "dstBlock": {
                    "startByte": 13,
                    "endByte": 24,
                    "startRow": 1,
                    "startCol": 0,
                    "endRow": 1,
                    "endCol": 11
                }
            }
            }
            "###),
            @r###"
        {
          "status": "error",
          "result": "Failed to find src item"
        }
        "###
        );

        insta::assert_json_snapshot!(handle_request(
                r###"
            {
            "method": "moveBlock",
            "params": {
                "queries": [
                "(function_item) @ident"
                ],
                "text": "fn main() {}\nfn foo() {}",
                "language": "rust",
                "srcBlock": {
                    "startByte": 0,
                    "endByte": 12,
                    "startRow": 0,
                    "startCol": 0,
                    "endRow": 0,
                    "endCol": 12
                },
                "dstBlock": {
                    "startByte": 999,
                    "endByte": 999,
                    "startRow": 999,
                    "startCol": 999,
                    "endRow": 999,
                    "endCol": 999
                }
            }
            }
            "###),
            @r###"
        {
          "status": "error",
          "result": "Failed to find dst item"
        }
        "###
        );
    }
}