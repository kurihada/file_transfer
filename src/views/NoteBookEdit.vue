<template>
    <el-row
        style="height: 100%; width: 100%; overflow: auto; background-color: #d9d9d9"
        :gutter="20">
        <el-col
            class="left-tree"
            :span="5"
            style="height: 100%; padding: 5px"
            @contextmenu.prevent="handleRightClick">
            <el-tree
                v-if="notebooks_data.length > 0"
                :data="notebooks_data"
                node-key="id"
                :highlight-current="true"
                @node-click="handleNodeClick"
                @node-expand="handleExpand"
                @node-collapse="handleCollapse"
                @node-contextmenu="handleContextmenu"
                style="font-size: medium">
                <template #default="{ data }">
                    <el-icon
                        v-if="data.is_dir"
                        style="display: flex; justify-content: center; align-items: center"
                        ><Folder
                    /></el-icon>
                    <el-icon
                        v-else
                        style="display: flex; justify-content: center; align-items: center"
                        ><Document
                    /></el-icon>
                    <div
                        style="
                            margin-left: 5px;
                            white-space: nowrap;
                            overflow: hidden;
                            text-overflow: ellipsis;
                        ">
                        {{ data.name }}
                    </div>
                </template>
            </el-tree>
            <div
                v-else-if="data.rootFolderPath != null"
                style="
                    height: 100%;
                    width: 100%;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                ">
                <el-icon size="large"><Plus /></el-icon>
                <el-text size="large">右键创建文件/文件夹</el-text>
            </div>
            <div
                v-else
                class="upload-container"
                @click="selectFolder"
                style="
                    height: 100%;
                    width: 100%;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                ">
                <el-icon size="large"><Plus /></el-icon>
                <el-text size="large">选择文件夹</el-text>
            </div>
            <context-menu v-model:show="context_menu_show" :options="optionsComponent">
                <context-menu-item label="刷新" @click="get_document_notebooks()" />
                <context-menu-group label="笔记">
                    <context-menu-item label="添加笔记" @click="showAddNotebookModalForm" />
                    <context-menu-item label="删除笔记" @click="showAddNotebookModalForm" />
                </context-menu-group>
                <context-menu-group label="文件夹">
                    <context-menu-item
                        label="添加文件夹"
                        @click="
                            () => {
                                data.addFolderModalFormVisible = true;
                            }
                        " />
                    <!-- <context-menu-item label="删除文件夹" @click="showAddFolderModalForm" /> -->
                </context-menu-group>
            </context-menu>
        </el-col>

        <!-- 编辑器栏 -->
        <el-col class="right-content" :span="19" style="background-color: #ffffff">
            <div v-if="data.showMdEditor">
                <mavon-editor v-model="data.mdText" height="94vh" @save="saveMdText" />
            </div>
        </el-col>
    </el-row>
    <!-- 添加文件夹弹出表单 -->
    <el-dialog
        title="添加文件夹"
        v-model="data.addFolderModalFormVisible"
        @close="handleCancelByAddFolderModal"
        center>
        <template #footer>
            <el-button @click="handleCancelByAddFolderModal">取消</el-button>
            <el-button
                type="primary"
                :loading="data.loading"
                @click="addNodeFolder(data.currentFolder, data.newFolderName)"
                >确认</el-button
            >
        </template>
        <div>
            <br />
            <br />
            <el-input v-model="data.newFolderName" placeholder="文件夹名称" clearable />
            <br />
            <br />
        </div>
    </el-dialog>
    <!-- 添加笔记弹出表单 -->
    <el-dialog
        title="添加笔记"
        v-model="data.addNotebookModalFormVisible"
        @close="handleCancelByAddNotebookModal"
        center>
        <template #footer>
            <el-button @click="handleCancelByAddNotebookModal">取消</el-button>
            <el-button
                type="primary"
                :loading="data.loading"
                @click="createNoteFile(data.currentFolder, data.newNoteMdFileName)"
                >确认</el-button
            >
        </template>
        <div>
            <br />
            <br />
            <el-input v-model="data.newNoteMdFileName" placeholder="笔记名称" clearable />
            <br />
            <br />
        </div>
    </el-dialog>
</template>
<script lang="ts" setup>
import { computed, onMounted, reactive, ref } from 'vue';
import DirFileMenu from '../utils/DirFileMenu.ts';
import StringUtil from '../utils/StringUtil.ts';
import NotificationUtil from '../utils/NotificationUtil.ts';
import { invoke } from '@tauri-apps/api/core';
import { ElMessageBox } from 'element-plus';
import { FileInfo } from '../entity/FileInfo.ts';
import { open } from '@tauri-apps/plugin-dialog';
import MarkdownFile from '../utils/MarkdownFile.ts';
import { NOT_FOUND_CODE } from '../entity/Response.ts';
import { tr } from 'element-plus/es/locale/index';

const handleRightClick = (event: MouseEvent) => {
    event.preventDefault();
    context_menu_show.value = true;
    optionsComponent.x = event.x;
    optionsComponent.y = event.y;
};

const selectFolder = async () => {
    try {
        const selected = await open({
            directory: true,
            multiple: false,
        });

        if (selected && typeof selected === 'string') {
            invoke('set_data_dir', { dataDir: selected })
                .then((_: any) => {
                    NotificationUtil.success('选择完成');
                    get_document_notebooks();
                })
                .catch((err) => {
                    NotificationUtil.error('文件夹选择失败:' + err);
                });
        }
    } catch (error) {
        console.error('Error selecting folder:', error);
    }
};

onMounted(() => {
    get_document_notebooks();
});

const handleNodeClick = (tree: FileInfo) => {
    if (tree.is_file) {
        let key = tree.name;
        data.currentNote = key;
        //保存原来编辑笔记
        if (
            data.mdTitle != key &&
            StringUtil.isNotNullAndEmpty(data.mdTitle) &&
            StringUtil.isNotNullAndEmpty(key)
        ) {
            ElMessageBox.confirm('保存笔记', '', {
                confirmButtonText: '保存',
                cancelButtonText: '不保存',
                message: '上一份笔记未保存，需要保存吗?',
                type: 'warning',
                showClose: false,
                draggable: true,
            })
                .then(() => {
                    invoke('get_note_list', { notebook: data.currentOldNotebook })
                        .then((res: any) => {
                            let same_file_name = false;
                            if (res && res.code == 200) {
                                var tem = res.data;
                                tem.map((n: any) => {
                                    if (n == data.mdTitle && data.mdTitle != data.mdOldTitle) {
                                        same_file_name = true;
                                    }
                                });
                                if (!same_file_name) {
                                    Promise.all([saveMd(data.mdText, data.mdTitle)]).then(() => {
                                        //点击了确定，开始删除
                                        invoke('remove_note', {
                                            currentNotebook: data.currentOldNotebook,
                                            currentNote: data.mdOldTitle + '.md',
                                        })
                                            .then((res: any) => {
                                                if (res && res.code == 200) {
                                                    get_document_notebooks();
                                                    get_note_list();
                                                } else {
                                                    NotificationUtil.error('删除旧文件操作失败!');
                                                }
                                            })
                                            .catch((err) => {
                                                console.log(err);
                                            });
                                        readNoteFile(tree.name, tree.path);
                                    });
                                } else {
                                    NotificationUtil.warning('已有相同文件名，文件名将无法更改！');
                                    Promise.all([saveMd(data.mdText, data.mdOldTitle)]).then(() => {
                                        invoke('read_note_file', {
                                            curPath: tree.path,
                                        })
                                            .then((res: any) => {
                                                if (res && res.code == 200) {
                                                    var tem = res.data;
                                                    data.mdText = tem;
                                                    data.notEditedMdtext = tem; // 原始笔记内容
                                                    data.mdTitle = key;
                                                    data.mdOldTitle = key;
                                                    data.currentOldNotebook = data.currentFolder;
                                                    data.showMdEditor = true;
                                                } else {
                                                    NotificationUtil.error('读取失败：' + res.msg);
                                                }
                                            })
                                            .catch((err) => {
                                                NotificationUtil.error(err);
                                            });
                                    });
                                }
                            }
                        })
                        .catch((err) => {
                            NotificationUtil.error('获取数据失败 ：' + err);
                        });
                })
                .catch(() => {
                    readNoteFile(tree.name, tree.path);
                });
        } else {
            readNoteFile(tree.name, tree.path);
        }
    }
};

const readNoteFile = (title: string, path: string) => {
    invoke('read_note_file', {
        curPath: path,
    })
        .then((res: any) => {
            console.log(res);
            if (res && res.code == 200) {
                var tem = res.data;
                data.mdText = tem;
                data.mdTitle = title;
                data.showMdEditor = true;
                const lastSlashIndex = path.lastIndexOf('/');
                if (lastSlashIndex !== -1) {
                    data.currentOldNotebook = path.substring(0, lastSlashIndex);
                }
            } else {
                NotificationUtil.error('读取失败：' + res.msg);
            }
        })
        .catch((err) => {
            NotificationUtil.error(err);
        });
};

const handleExpand = (tree: FileInfo) => {};

const handleCollapse = (tree: FileInfo) => {};

const handleContextmenu = (event: MouseEvent, tree: FileInfo) => {
    context_menu_show.value = true;
    optionsComponent.x = event.x;
    optionsComponent.y = event.y;
    data.currentFolder = tree.path;
};

let context_menu_show = ref(false);
const optionsComponent = {
    theme: 'mac',
    zIndex: 3,
    minWidth: 150,
    x: 500,
    y: 200,
};

let notebooks_data = ref<FileInfo[]>([]);
/**
 *  获取笔记本数据
 */
const get_document_notebooks = () => {
    MarkdownFile.getNotebooks()
        .then((fileInfo: FileInfo) => {
            console.log(fileInfo);
            if (fileInfo !== undefined && fileInfo.children !== undefined) {
                notebooks_data.value = fileInfo.children;
                data.rootFolderPath = fileInfo.path;
                data.currentFolder = fileInfo.path;
            }
        })
        .catch((error) => {
            if (NOT_FOUND_CODE === error.code) {
                NotificationUtil.warning('请选择文件夹');
                return;
            }
            NotificationUtil.error('获取数据失败: ' + error);
        });
};

/**
 * 文件夹及文件夹下笔记信息
 */
const show_notebooks_data = computed(() => {
    return DirFileMenu.dir_file(notebooks_data.value);
});

/**
 * 保存笔记 切换笔记但未保存时调用
 * @param {笔记内容} mdText
 * @param {标题} mdTitle
 */
const saveMd = (mdText: any, mdTitle: any) => {
    var p = new Promise(function () {
        if (data.mdTitle == '') {
            // 标题为空
            NotificationUtil.error('请输入标题');
            return;
        }

        if (data.currentOldNotebook && data.currentNote) {
            // 不为空
            // 调用保存笔记方法
            invoke('save_note', {
                currentNotebook: data.currentOldNotebook,
                currentNote: data.currentNote,
                mdTitle: mdTitle,
                mdText: mdText,
            })
                .then((res: any) => {
                    if (res && res.code == 200) {
                        NotificationUtil.success('笔记保存成功');
                    } else {
                        NotificationUtil.error('笔记保存失败：' + res.msg);
                    }
                })
                .then((err) => {
                    console.log(err);
                });
        } else {
            // 存在为空情况
            NotificationUtil.error('请选定笔记本和笔记');
        }
        return p;
    });
};

/**
 * 保存文章/笔记 编辑器点击保存
 * @param {*} mdText
 */
const saveMdText = (mdText: string, render: string) => {
    if (data.mdTitle == '') {
        // 标题为空
        NotificationUtil.error('请输入标题');
        return;
    }
    invoke('get_note_list', { notebook: data.currentOldNotebook })
        .then((res: any) => {
            let same_file_name = false;
            if (res && res.code == 200) {
                var tem = res.data;
                tem.map((n: any) => {
                    if (n == data.mdTitle && data.mdTitle != data.mdOldTitle) {
                        same_file_name = true;
                    }
                });
                if (!same_file_name) {
                    if (data.currentOldNotebook && data.currentNote) {
                        // 不为空
                        // 调用保存笔记方法
                        invoke('save_note', {
                            currentNotebook: data.currentOldNotebook,
                            currentNote: data.currentNote,
                            mdTitle: data.mdTitle,
                            mdText: mdText,
                        })
                            .then((res: any) => {
                                if (res && res.code == 200) {
                                    NotificationUtil.success('笔记保存成功');
                                    //新文件和旧文件名不同，需要将旧文件删除
                                    if (
                                        data.mdTitle != data.mdOldTitle &&
                                        StringUtil.isNotNullAndEmpty(data.mdTitle) &&
                                        StringUtil.isNotNullAndEmpty(data.mdOldTitle)
                                    ) {
                                        //点击了确定，开始删除
                                        invoke('remove_note', {
                                            currentNotebook: data.currentOldNotebook,
                                            currentNote: data.mdOldTitle + '.md',
                                        })
                                            .then((res: any) => {
                                                if (res && res.code == 200) {
                                                    data.currentNote = data.mdTitle;
                                                    data.mdOldTitle = data.mdTitle;
                                                    data.notEditedMdtext = mdText;
                                                    get_document_notebooks();
                                                    get_note_list();
                                                } else {
                                                    NotificationUtil.error('删除旧文件操作失败!');
                                                }
                                            })
                                            .catch((err) => {
                                                console.log(err);
                                            });
                                    }
                                } else {
                                    NotificationUtil.error('笔记保存失败：' + res.msg);
                                }
                            })
                            .then((err) => {
                                console.log(err);
                            });
                    } else {
                        // 存在为空情况
                        NotificationUtil.error('请选定笔记本和笔记');
                    }
                } else {
                    NotificationUtil.warning('已有相同文件名，文件名将无法更改！');
                    if (data.currentOldNotebook && data.currentNote) {
                        // 调用保存笔记方法 保存新文件
                        invoke('save_note', {
                            currentNotebook: data.currentOldNotebook,
                            currentNote: data.currentNote,
                            mdTitle: data.mdOldTitle,
                            mdText: mdText,
                        }).then((res: any) => {
                            if (res && res.code == 200) {
                                data.mdTitle = data.mdOldTitle;
                                data.notEditedMdtext = mdText;
                            } else {
                                NotificationUtil.error('笔记保存失败：' + res.msg);
                            }
                        });
                    } else {
                        // 存在为空情况
                        NotificationUtil.error('请选定笔记本和笔记');
                    }
                }
            } else {
                NotificationUtil.error('获取笔记列表数据失败：' + res.msg);
            }
        })
        .catch((err) => {
            console.log(err);
            NotificationUtil.error('获取文件信息异常！');
            return;
        });
};

/**
 * 添加文件夹
 */
const addNodeFolder = (parentFolderName: string, folderName: string) => {
    console.log('添加文件夹：' + parentFolderName + '  ' + folderName);
    folderName = folderName.trim();
    if (StringUtil.isNullAndEmpty(folderName)) {
        NotificationUtil.error('创建文件夹失败，请填写文件夹名称！');
        return;
    }
    if (StringUtil.isNullAndEmpty(parentFolderName)) {
        NotificationUtil.error('创建文件夹失败，父文件夹异常');
        return;
    }
    if (!StringUtil.namingVerify(folderName)) {
        return false;
    }
    data.confirmLoading = true;
    data.loading = true;
    // 创建文件夹
    MarkdownFile.createFolder(parentFolderName + '/' + folderName)
        .then((_: any) => {
            NotificationUtil.success('文件夹创建成功');
            data.addFolderModalFormVisible = false; // 隐藏模态表单
            get_document_notebooks();
            data.confirmLoading = false; // 停止转圈圈
            data.loading = false;
        })
        .catch((err) => {
            NotificationUtil.error('文件夹创建失败' + err);
        });
    data.currentFolder = data.rootFolderPath;
};
// 显示添加笔记对话框
const showAddNotebookModalForm = () => {
    if (StringUtil.isNullAndEmpty(data.currentFolder)) {
        NotificationUtil.error(`请选择文件所在的文件夹`);
        return;
    }
    data.addNotebookModalFormVisible = true;
};
/**
 * 添加笔记
 */
const createNoteFile = async (folderPath: string, fileName: string) => {
    MarkdownFile.createNoteFile(folderPath, fileName).then((_: void) => {
        NotificationUtil.success('文件创建成功');
        data.addNotebookModalFormVisible = false; // 隐藏模态表单
        data.currentFolder = data.rootFolderPath;
        get_document_notebooks();
    });
};

/**
 * 关闭添加文件夹模态框
 */
const handleCancelByAddFolderModal = () => {
    data.addFolderModalFormVisible = false;
    data.newFolderName = '';
    data.currentFolder = data.rootFolderPath;
};
/**
 * 关闭添加笔记模态框
 */
const handleCancelByAddNotebookModal = () => {
    data.addNotebookModalFormVisible = false;
    data.newNoteMdFileName = '';
    data.currentFolder = data.rootFolderPath;
};

const data = reactive({
    rootFolderPath: '',
    currentFolder: '', // 当前选中的文件夹
    currentOldNotebook: '', //上次选中的文件夹
    currentNote: '', // 当前选中笔记
    notEditedMdtext: '', // 未编辑的文章内容
    showMdEditor: false, // 是否显示编辑器
    newFolderName: '', // 新建笔记本的名称
    addNotebookModalFormVisible: false, // 是否显示添加笔记本表单
    addFolderModalFormVisible: false,
    confirmLoading: false, // 添加笔记表单等待状态（是否转圈圈）
    newNoteMdFileName: '', // 新建的笔记文件名
    mdTitle: '', // 文章标题
    mdOldTitle: '', // 文章旧标题标题
    mdText: '', // 文章内容Markdown格式
    loading: false,
    map: new Map(),
    spinning: false,
    show_notebooks_data,
});
</script>
<style>
.el-tree-node {
    padding: 6px;
}
</style>
