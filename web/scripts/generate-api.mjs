// npm run api:generate — exports the OpenAPI description of the host, then regenerates the TypeScript types.
// The output path is absolute: `dotnet run` on a web project runs from the project folder, not from here.
import { execFileSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const web = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const output = resolve(web, "openapi", "openapi.json");
const run = (command, args) => execFileSync(command, args, { cwd: web, stdio: "inherit", shell: process.platform === "win32" });

run("dotnet", ["run", "--project", resolve(web, "..", "src", "Educator.Host"), "--", "--export-openapi", output]);
run("npm", ["run", "api:types"]);
