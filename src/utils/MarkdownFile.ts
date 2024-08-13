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
            await invoke<Response<String>>('create_folder', {
                folderPath: folderPath,
            });
        } catch (error) {
            throw error;
        }
    },

    async createNoteFile(folderPath: string, fileName: string) {
        try {
            await invoke<Response<String>>('create_note_file', {
                folderPath: folderPath,
                fileName: fileName,
            });
        } catch (error) {
            throw error;
        }
    },
};

export default _;
