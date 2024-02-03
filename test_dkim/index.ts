import { dkim_1_backend } from "./dkim_1_backend/index";
import fs from 'fs/promises';

async function readFileAsString(filePath: string) {
    try {
        const data = await fs.readFile(filePath, 'utf8');
        return data;
    } catch (error: any) {
        console.error(`Error reading file: ${error.message}`);
        throw error; // Re-throw the error for further handling if needed
    }
}
async function main() {
    const filePath = 'txt.txt';
    try {
        const fileContents = await readFileAsString(filePath);
        console.log({ dkim_1_backend })
        const a = await dkim_1_backend.run_command(fileContents);
        console.log(a, "resultss")
        console.log(fileContents);
    } catch (error: any) {
        console.error(`Error handling file contents: ${error.message}`);
    }

}

main().catch((err) => { console.error(err); process.exit(1); })