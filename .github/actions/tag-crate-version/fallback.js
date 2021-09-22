const fs = require("fs");
const path = require('path');
const TOML = require('@ltd/j-toml');

const toml_opts = { multi: true };
const MANIFEST_FILE_NAME = "Cargo.toml";


function read_manifest(manifest_path) {
	if (fs.existsSync(manifest_path)) {
		const content = fs.readFileSync(manifest_path);
		const result = read_manifest_source(content.toString(), manifest_path)
		return result;
	}
}

function read_manifest_source(source, path) {
	const toml = TOML.parse(source, '\n', false, xOptions = toml_opts);
	let result = read_manifest_data(toml, path);

	result.toml = toml;
	result.source = source;

	return result;
}

function read_manifest_data(manifest, path) {
	// if (manifest.package?.version) {
	if (manifest && manifest.package && manifest.package.version) {
		return {
			path: path,
			name: manifest.package.name,
			version: manifest.package.version,
		}
	}
}

let cached_root = undefined;
const DEFAULT_PWD = ".";

function get_root(pwd = DEFAULT_PWD) {
	// read root manifest file:
	const root = read_manifest(path.join(pwd, MANIFEST_FILE_NAME));
	cached_root = root;
	return root;
}

function get_workspace_members(pwd = DEFAULT_PWD) {
	const root = cached_root || get_root(pwd);
	const root_toml = root.toml;
	let crates = Object();
	crates[root.name] = root;
	// read workspace members from root manifest file:
	// for (i in root_toml.workspace?.members) {
	if (root_toml && root_toml.workspace && root_toml.workspace.members) {
		for (i in root_toml.workspace.members) {
			const crate_path = root_toml.workspace.members[i];
			const manifest = path.join(pwd, crate_path, MANIFEST_FILE_NAME);
			const crate = read_manifest(manifest);
			crates[crate.name] = crate;
		}
	}
	return crates;
}

exports.get_root = get_root;
exports.get_workspace_members = get_workspace_members;
