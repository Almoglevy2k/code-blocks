import * as vscode from "vscode";
import { notify, openDocument, sleep } from "../exampleUtils";
import { TreeViewer } from "../../TreeViewer";
import { expect } from "chai";

const TIMEOUT = process.env.EXAMPLE_TIMEOUT ?? "2m";
test("Tree viewer", async function () {
    await notify("Open any file");

    await openDocument({
        language: "rust",
        content: `
#[derive(Debug)]
struct A {
    /// b property
    b: u32
}

fn main() {

}
`,
    });

    await sleep(1500);
    await notify("Call the 'codeBlocks.openTreeViewer' command");
    await sleep(1500);

    await vscode.commands.executeCommand("codeBlocks.openTreeViewer");
    const treeViewerDocument = await vscode.workspace.openTextDocument(TreeViewer.uri);
    while (treeViewerDocument.getText() === TreeViewer.placeholder) {
        await new Promise<void>((r) => TreeViewer.treeViewer.onDidChange(() => r()));
    }

    expect("\n" + treeViewerDocument.getText()).to.be.equal(`
source_file [1:0 - 10:0]
  attribute_item [1:0 - 1:16]
    attribute [1:2 - 1:15]
      identifier [1:2 - 1:8]
      token_tree [1:8 - 1:15]
        identifier [1:9 - 1:14]
  struct_item [2:0 - 5:1]
    type_identifier [2:7 - 2:8]
    field_declaration_list [2:9 - 5:1]
      line_comment [3:4 - 3:18]
      field_declaration [4:4 - 4:10]
        field_identifier [4:4 - 4:5]
        primitive_type [4:7 - 4:10]
  function_item [7:0 - 9:1]
    identifier [7:3 - 7:7]
    parameters [7:7 - 7:9]
    block [7:10 - 9:1]`);

    await sleep(1500);

    await notify("Opening a different document updates the tree");

    await sleep(1500);

    await openDocument({
        language: "typescriptreact",
        content: `
function main() {
    return (
        <>
            <div>hello world</div>
        </>
    )
}
    `,
    });

    const didChange = (): boolean =>
        treeViewerDocument.getText() !== TreeViewer.placeholder &&
        !treeViewerDocument.getText().startsWith("source_file");

    if (didChange()) {
        await new Promise<void>((r) =>
            TreeViewer.treeViewer.onDidChange(() => {
                if (!didChange()) {
                    r();
                }
            })
        );
    }

    await sleep(100);

    expect("\n" + treeViewerDocument.getText()).to.be.equal(`
program [1:0 - 8:4]
  function_declaration [1:0 - 7:1]
    identifier [1:9 - 1:13]
    formal_parameters [1:13 - 1:15]
    statement_block [1:16 - 7:1]
      return_statement [2:4 - 6:5]
        parenthesized_expression [2:11 - 6:5]
          jsx_fragment [3:8 - 5:11]
            jsx_text [3:10 - 4:12]
            jsx_element [4:12 - 4:34]
              jsx_opening_element [4:12 - 4:17]
                identifier [4:13 - 4:16]
              jsx_text [4:17 - 4:28]
              jsx_closing_element [4:28 - 4:34]
                identifier [4:30 - 4:33]
            jsx_text [4:34 - 5:8]`);

    await sleep(1500);
}).timeout(TIMEOUT);