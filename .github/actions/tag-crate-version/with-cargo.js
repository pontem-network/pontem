const util = require('util');
const exec = util.promisify(require('child_process').exec);


function read_meta_line(input, index) {
	let line = input.trim();
	if (line.startsWith('"')) line = line.substring(1);
	if (line.endsWith('"')) line = line.substring(0, line.length - 1);

	const info = line.split(' ');
	const crate = info[0];
	const version = info[1];
	const uri = info[2];

	return { name: crate, version: version, uri: uri }
}

async function get_root(pwd) {
	try {
		const { err, stdout, stderr } = await exec("cargo metadata --format-version=1 --quiet | jq '.resolve.root'", { cwd: pwd });
		if (err) { return fail(err); }
		return read_meta_line(stdout);
	} catch (error) {
		fail(error.message);
	}
}

async function get_workspace_members(pwd) {
	try {
		const { err, stdout, stderr } = await exec("cargo metadata --format-version=1 --quiet | jq '.workspace_members[]'", { cwd: pwd });
		if (err) { return fail(err); }
		let crates = [];
		stdout.trim().split('\n').forEach(line => { crates.push(read_meta_line(line)); });
		return crates;
	} catch (error) {
		fail(error.message);
	}
}


const DEFAULT_INPUT_CRATE_NAME = undefined;
const DEFAULT_TAG_TO_VERSION = "v?([0-9]+.[0-9]+.*)";
const DEFAULT_VERSION_TO_TAG = "v$1";
const DEFAULT_GITHUB_TOKEN = undefined;
const DEFAULT_PWD = undefined;


function get_github_action_inputs() {
	try {
		const core = require('@actions/core');
		try {
			const optional = { required: false, trimWhitespace: true };
			const pwd = core.getInput('pwd', optional) || DEFAULT_PWD;
			const input_crate_name = core.getInput('crate', optional) || DEFAULT_INPUT_CRATE_NAME;
			const tag_to_version = core.getInput('tag-to-version', optional) || DEFAULT_TAG_TO_VERSION;
			const version_to_tag = core.getInput('version-to-tag', optional) || DEFAULT_VERSION_TO_TAG;
			const github_token = core.getInput('token', optional) || DEFAULT_GITHUB_TOKEN;
			return { pwd, input_crate_name, tag_to_version, version_to_tag, github_token }
		} catch (error) {
			fail(error.message);
		}
	} catch (error) {
		fail(error.message);
	}

	return {
		pwd: DEFAULT_PWD,
		input_crate_name: DEFAULT_INPUT_CRATE_NAME,
		tag_to_version: DEFAULT_TAG_TO_VERSION,
		version_to_tag: DEFAULT_VERSION_TO_TAG,
		github_token: DEFAULT_GITHUB_TOKEN,
	}
}

function fail(error) {
	try {
		const core = require('@actions/core');
		core.setFailed(error);
	} catch (_error) {
		console.error(error);
	}
}


exports.get_root = get_root;
exports.get_workspace_members = get_workspace_members;
