import * as vscode from 'vscode';
import { renderString } from 'base9-builder-node';
import * as fs from 'fs';
import * as path from 'path';
import yaml from 'yaml';

const PALETTE = 'base9.palette';
const themePath: string = path.resolve(__dirname, '../themes/base9.json');
const templatePath: string = path.resolve(__dirname, '../src/base9.mustache.yml');

// const semanticFile: string = path.resolve(__dirname, '../src/semantic.yml');



// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
	if(!fs.existsSync(themePath)) {
		generateTheme();
	}
	fs.watchFile(templatePath, {interval: 1000}, (e, n) => {
		console.log("template changed");
		generateTheme();
	});
	// fs.watchFile(semanticFile, {interval: 800}, (e, n) => {
	// 	console.log("semantic changed");
	// 	generateTheme();
	// });

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	context.subscriptions.push(vscode.workspace.onDidChangeConfiguration(e => {
		if (e.affectsConfiguration(PALETTE)) {
			generateTheme();
		}
	}));

	// The command has been defined in the package.json file
	// Now provide the implementation of the command with registerCommand
	// The commandId parameter must match the command field in package.json
	let generate = vscode.commands.registerCommand('base9-vscode.generate', () => {
		// showInformationMessage('Hello World!');
		// The code you place here will be executed every time your command is executed
		// Display a message box to the user
		generateTheme();
	});

	context.subscriptions.push(generate);
}

// this method is called when your extension is deactivated
export function deactivate() {}

async function generateTheme(palette?: string): Promise<void> {
	if(palette === undefined) {
			palette = vscode.workspace.getConfiguration().get(PALETTE) as string;
	}

	const template = fs.readFileSync(templatePath, {encoding: 'utf8'});

	const outputYaml = renderString(palette, template);
	const output = JSON.stringify(yaml.parse(outputYaml), undefined, "  ");

	fs.writeFileSync(themePath, output);

	await promptRestart("Please Restart.");
}

async function promptRestart(informationMessage: string): Promise<void> {
    let reloadAction: vscode.MessageItem = {title: 'Reload Now'};
    let selectedAction = await vscode.window.showInformationMessage(informationMessage, reloadAction);
    if (!selectedAction) {
        return;
    }
    if (selectedAction.title == reloadAction.title) {
        await vscode.commands.executeCommand('workbench.action.reloadWindow');
    }
}
