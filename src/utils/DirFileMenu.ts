import { FolderOpened, Document } from '@element-plus/icons-vue';
import { h, VNode } from 'vue';
import StringUtil from './StringUtil';
import { FileInfo } from '../entity/FileInfo';
interface GetItem {
    parent_id: string | number;
    is_file: boolean;
    path: string;
    label: string;
    key: string | number;
    icon: () => VNode;
    children?: GetItem[];
    type?: string;
    name: string;
}

let _ = {
    dir_file(children_data: FileInfo[]): GetItem[] {
        let data: GetItem[] = [];
        for (let i in children_data) {
            let parent_id = children_data[i].parent_path;
            let label = children_data[i].name;
            let key = children_data[i].id;
            let is_file = children_data[i].is_file;
            let path = children_data[i].path;
            let children = children_data[i].children;
            let children_: GetItem[] = [];
            if (children != null && children !== undefined) {
                children_ = _.dir_file(children);
            }
            if (!is_file) {
                data.push(
                    _.getItem(
                        parent_id,
                        is_file,
                        path,
                        label,
                        key,
                        () => h(FolderOpened),
                        children_,
                    ),
                );
            } else {
                data.push(_.getItem(parent_id, is_file, path, label, key, () => h(Document)));
            }
        }
        return data;
    },

    getItem(
        parent_id: string | number,
        is_file: boolean,
        path: string,
        label: string,
        key: string | number,
        icon: () => VNode,
        children?: GetItem[],
        type?: string,
    ): GetItem {
        let name = '';
        if (StringUtil.isNotNullAndEmpty(label)) {
            let arr = label.split('.');
            if (arr.length > 1) {
                name = arr[0];
            }
        }
        return {
            parent_id,
            path,
            is_file: is_file,
            key,
            icon,
            children,
            label,
            type,
            name,
        };
    },

    /**
     * 获取目录下文件，在创建新文件时使用，验证是否同名 返回Set集合
     * children_data 目录及文件数据  openkey 打开的文件夹id
     */
    getFolderChilrenFileNameSet(children_data: FileInfo[], file: string): Set<string> {
        let data = new Set<string>();
        for (let i in children_data) {
            let id = children_data[i].id;
            if (openkey === id) {
                let children = children_data[i].children;
                if (children) {
                    for (let j in children) {
                        let name_ = children[j].name;
                        let array = name_.split('.');
                        if (array[0] !== null && array[0] !== '' && array[0] !== undefined) {
                            data.add(array[0]);
                        }
                    }
                }
            }
        }
        return data;
    },

    /**
     * 获取文件夹名称
     * @returns
     */
    getFolderNameSet(children_data: FileInfo[]): Set<string> {
        let data = new Set<string>();
        for (let i in children_data) {
            let name = children_data[i].name;
            if (name !== null && name !== '' && name !== undefined) {
                data.add(name);
            }
        }
        return data;
    },
};

export default _;
