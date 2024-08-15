import { invoke } from '@tauri-apps/api/core';
import { FileInfo } from '../entity/FileInfo';
import { Response } from '../entity/Response';

let _ = {
    async getNotebooks(): Promise<FileInfo> {
        try {
            const result: Response<FileInfo> = await invoke<Response<FileInfo>>(
                'get_document_notebooks',
            );
            console.log(result);
            console.log(result.data);
            // const a = JSON.parse(result) as Response<FileInfo>;
            return result.data;
        } catch (error) {
            throw error;
        }
    },

    async createFolder(folderPath: string) {
        try {
            await invoke<Response<string>>('create_folder', {
                folderPath: folderPath,
            });
        } catch (error) {
            throw error;
        }
    },

    async createNoteFile(folderPath: string, fileName: string) {
        try {
            await invoke<Response<string>>('create_note_file', {
                folderPath: folderPath,
                fileName: fileName,
            });
        } catch (error) {
            throw error;
        }
    },

    async saveNote(folderPath: string, fileName: string, fileContent: string) {
        try {
            console.log(fileName);
            await invoke<Response<string>>('save_note', {
                folderPath: folderPath,
                fileName: fileName,
                fileContent: fileContent,
            });
        } catch (error) {
            throw error;
        }
    },

    async readNoteFile(filePath: string): Promise<string> {
        try {
            console.log(filePath);
            let result = await invoke<Response<string>>('read_note_file', {
                curPath: filePath,
            });
            console.log(result);
            return result.data;
        } catch (error) {
            throw error;
        }
    },
};

export default _;
