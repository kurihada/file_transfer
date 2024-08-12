import { invoke } from '@tauri-apps/api/core';
import NotificationUtil from './NotificationUtil';
import StringUtil from './StringUtil';
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
    },

    async deleteNotebook(filePath: string): Promise<boolean> {},
};

export default _;
