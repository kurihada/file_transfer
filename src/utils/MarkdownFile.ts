import { invoke } from '@tauri-apps/api/core';
import NotificationUtil from './NotificationUtil';
import StringUtil from './StringUtil';
import { FileInfo } from '../entity/FileInfo';

let _ = {
    async getNotebooks(): Promise<FileInfo> {
        try {
            const result = await invoke<string>('get_document_notebooks');
            const fileInfo: FileInfo = JSON.parse(result) as FileInfo;
            return fileInfo;
        } catch (error) {
            NotificationUtil.error('获取数据失败: ' + error);
            throw error;
        }
    },

    /**
     * 添加笔记
     * @param curDir 当前文件夹
     * @param fileName 笔记名称
     * @returns 是否添加成功
     */
    async addNotebook(curDir: string, fileName: string): Promise<boolean> {
        if (StringUtil.isNullAndEmpty(curDir)) {
            NotificationUtil.error(`请选择文件所在的文件夹`);
            return false;
        }

        fileName = fileName.trim();
        if (StringUtil.isNullAndEmpty(fileName)) {
            NotificationUtil.error('创建笔记失败，请填写笔记名称！');
            return false;
        }

        if (!StringUtil.namingVerify(fileName)) {
            return false;
        }

        try {
            const existFile: boolean = await invoke<boolean>('exist_file', {
                curDir: curDir,
                fileName: fileName,
            });
            if (existFile) {
                NotificationUtil.error('已有相同文件名，请检查！');
                return false;
            }
            const createFileName = await invoke<string>('create_note_file', {
                curDir: curDir,
                fileName: fileName,
            });
            NotificationUtil.success('笔记创建成功：' + createFileName);
            return true;
        } catch (error) {
            NotificationUtil.error('笔记创建失败: ' + error);
            return false;
        }
    },

    async deleteNotebook(filePath: string): Promise<boolean> {},
};

export default _;
