<template>
    <el-container
        id="app-container"
        style="
            height: 100%;
            width: 100%;
            overflow: auto;
            background-color: #d9d9d9;
            border-radius: 8px;
        ">
        <el-aside
            style="height: 100%; background-color: #ffffff"
            :style="{
                width: `${globalData.sidebarWidth}%`,
                transition: globalData.isResizing ? 'none' : 'width 0.2s',
            }"
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
                style="width: 90%; height: 100%; margin: 0px !important">
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
                    <div style="margin-right: 20px">
                        <div placement="right" trigger="hover">
                            <span
                                v-if="globalData.notSaveNotes.has(data.path)"
                                class="orange-dot"></span>
                        </div>
                    </div>
                </template>
            </el-tree>
            <div
                v-else-if="globalData.rootFolderPath != null"
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
                                globalData.addFolderModalFormVisible = true;
                            }
                        " />
                    <!-- <context-menu-item label="删除文件夹" @click="showAddFolderModalForm" /> -->
                </context-menu-group>
            </context-menu>
        </el-aside>
        <div
            style="
                width: 2px;
                background-color: #fbf6f6;
                display: flex;
                justify-content: center;
                align-items: center;
            ">
            <el-icon style="transform: rotate(90deg)" @mousedown="startResizing"
                ><DCaret
            /></el-icon>
        </div>
        <!-- 编辑器栏 -->
        <el-main style="height: 100%; width: 70%; background-color: #ffffff">
            <div v-if="globalData.showMdEditor">
                <mavon-editor
                    v-model="globalData.noteContent"
                    height="94vh"
                    @save="
                        saveMdText(
                            globalData.currentFolder,
                            globalData.currentNote,
                            globalData.noteContent,
                        )
                    " />
            </div>
        </el-main>
    </el-container>
    <!-- 添加文件夹弹出表单 -->
    <el-dialog
        title="添加文件夹"
        v-model="globalData.addFolderModalFormVisible"
        @close="handleCancelByAddFolderModal"
        center>
        <template #footer>
            <el-button @click="handleCancelByAddFolderModal">取消</el-button>
            <el-button
                type="primary"
                :loading="globalData.loading"
                @click="addNodeFolder(globalData.currentFolder, globalData.newFolderName)"
                >确认</el-button
            >
        </template>
        <div>
            <br />
            <br />
            <el-input v-model="globalData.newFolderName" placeholder="文件夹名称" clearable />
            <br />
            <br />
        </div>
    </el-dialog>
    <!-- 添加笔记弹出表单 -->
    <el-dialog
        title="添加笔记"
        v-model="globalData.addNotebookModalFormVisible"
        @close="handleCancelByAddNotebookModal"
        center>
        <template #footer>
            <el-button @click="handleCancelByAddNotebookModal">取消</el-button>
            <el-button
                type="primary"
                :loading="globalData.loading"
                @click="createNoteFile(globalData.currentFolder, globalData.newNoteMdFileName)"
                >确认</el-button
            >
        </template>
        <div>
            <br />
            <br />
            <el-input v-model="globalData.newNoteMdFileName" placeholder="笔记名称" clearable />
            <br />
            <br />
        </div>
    </el-dialog>
</template>
<script lang="ts" setup>
import { onMounted, reactive, ref } from 'vue';
import StringUtil from '../utils/StringUtil.ts';
import NotificationUtil from '../utils/NotificationUtil.ts';
import { invoke } from '@tauri-apps/api/core';
import { FileInfo } from '../entity/FileInfo.ts';
import { open } from '@tauri-apps/plugin-dialog';
import MarkdownFile from '../utils/MarkdownFile.ts';
import { NOT_FOUND_CODE } from '../entity/Response.ts';

onMounted(() => {
    get_document_notebooks();
});

const startResizing = (event: MouseEvent) => {
    globalData.isResizing = true;
    const startX = event.clientX;
    const startWidthPercent = globalData.sidebarWidth;
    let appContainer = document.getElementById('app-container');
    if (appContainer === undefined || appContainer === null) {
        return;
    }
    const containerWidth = appContainer.clientWidth;

    const doResize = (moveEvent: MouseEvent) => {
        const deltaX = moveEvent.clientX - startX;
        const deltaPercent = (deltaX / containerWidth) * 100;
        let newWidthPercent = startWidthPercent + deltaPercent;
        newWidthPercent = Math.max(newWidthPercent, 10); // 最小宽度限制为 10%
        globalData.sidebarWidth = newWidthPercent;
    };

    const stopResizing = () => {
        globalData.isResizing = false;
        document.removeEventListener('mousemove', doResize);
        document.removeEventListener('mouseup', stopResizing);
    };

    document.addEventListener('mousemove', doResize);
    document.addEventListener('mouseup', stopResizing);
};

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

const handleNodeClick = (fileInfo: FileInfo) => {
    if (fileInfo.is_dir) {
        return;
    }
    if (fileInfo.name === globalData.currentNote) {
        return;
    }
    let notSaveNoteContent = globalData.notSaveNotes.get(fileInfo.path);
    if (
        notSaveNoteContent !== undefined &&
        notSaveNoteContent !== null &&
        notSaveNoteContent !== ''
    ) {
        globalData.noteContent = notSaveNoteContent;
        globalData.showMdEditor = true;
        globalData.currentNote = fileInfo.name;
        globalData.currentNotePath = fileInfo.path;
        return;
    }
    if (StringUtil.isNotNullAndEmpty(globalData.currentNotePath)) {
        let currentNotePath = globalData.currentNotePath;
        let noteContent = globalData.noteContent;
        MarkdownFile.readNoteFile(currentNotePath)
            .then((res: string) => {
                let oldNoteContent = res;
                if (noteContent !== oldNoteContent) {
                    globalData.notSaveNotes.set(currentNotePath, noteContent);
                } else {
                    globalData.notSaveNotes.delete(currentNotePath);
                }
            })
            .catch((err) => NotificationUtil.error('读取文件失败: ' + err));
    }

    readNoteFile(fileInfo.path);
    globalData.currentNote = fileInfo.name;
    globalData.currentNotePath = fileInfo.path;
};

const readNoteFile = (notePath: string) => {
    MarkdownFile.readNoteFile(notePath)
        .then((res: string) => {
            globalData.noteContent = res;
            globalData.showMdEditor = true;
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
    globalData.currentFolder = tree.path;
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
                globalData.rootFolderPath = fileInfo.path;
                globalData.currentFolder = fileInfo.path;
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
 * 保存文章/笔记 编辑器点击保存
 */
const saveMdText = (folderPath: string, fileName: string, fileContent: string) => {
    MarkdownFile.saveNote(folderPath, fileName, fileContent)
        .then((_: void) => {
            NotificationUtil.success('笔记保存成功');
        })
        .catch((error) => {
            NotificationUtil.error('笔记保存失败: ' + error);
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
    globalData.confirmLoading = true;
    globalData.loading = true;
    // 创建文件夹
    MarkdownFile.createFolder(parentFolderName + '/' + folderName)
        .then((_: any) => {
            NotificationUtil.success('文件夹创建成功');
            globalData.addFolderModalFormVisible = false; // 隐藏模态表单
            get_document_notebooks();
            globalData.confirmLoading = false; // 停止转圈圈
            globalData.loading = false;
        })
        .catch((err) => {
            NotificationUtil.error('文件夹创建失败' + err);
        });
    globalData.currentFolder = globalData.rootFolderPath;
};
// 显示添加笔记对话框
const showAddNotebookModalForm = () => {
    if (StringUtil.isNullAndEmpty(globalData.currentFolder)) {
        NotificationUtil.error(`请选择文件所在的文件夹`);
        return;
    }
    globalData.addNotebookModalFormVisible = true;
};
/**
 * 添加笔记
 */
const createNoteFile = async (folderPath: string, fileName: string) => {
    MarkdownFile.createNoteFile(folderPath, fileName).then((_: void) => {
        NotificationUtil.success('文件创建成功');
        globalData.addNotebookModalFormVisible = false; // 隐藏模态表单
        globalData.currentFolder = globalData.rootFolderPath;
        get_document_notebooks();
    });
};

/**
 * 关闭添加文件夹模态框
 */
const handleCancelByAddFolderModal = () => {
    globalData.addFolderModalFormVisible = false;
    globalData.newFolderName = '';
    globalData.currentFolder = globalData.rootFolderPath;
};
/**
 * 关闭添加笔记模态框
 */
const handleCancelByAddNotebookModal = () => {
    globalData.addNotebookModalFormVisible = false;
    globalData.newNoteMdFileName = '';
    globalData.currentFolder = globalData.rootFolderPath;
};

const globalData = reactive({
    sidebarWidth: 20,
    isResizing: false,
    rootFolderPath: '',
    currentFolder: '', // 当前选中的文件夹
    currentNotePath: '',
    currentNote: '', // 当前选中笔记
    showMdEditor: false, // 是否显示编辑器
    newFolderName: '', // 新建笔记本的名称
    addNotebookModalFormVisible: false, // 是否显示添加笔记本表单
    addFolderModalFormVisible: false,
    confirmLoading: false, // 添加笔记表单等待状态（是否转圈圈）
    newNoteMdFileName: '', // 新建的笔记文件名
    noteContent: '', // 旧文章内容
    loading: false,
    notSaveNotes: new Map<string, string>(),
});
</script>
<style>
.el-tree-node {
    padding: 6px;
}
/* 黄点样式 */
.orange-dot {
    display: inline-block;
    width: 8px;
    height: 8px;
    background-color: #ff7f00; /* 亮眼的橙色 */
    border-radius: 50%;
    margin-left: 10px; /* 可根据需要调整 */
}
/* 偶数行 */
.el-tree-node:nth-child(even) {
    background-color: #f9f9f9;
    border-radius: 8px;
}

/* 修改选中节点的背景颜色 */
.el-tree-node.is-current > .el-tree-node__content {
    background-color: #40a0ff33 !important; /* 这里设置为你需要的颜色 */
    color: #409eff; /* 修改字体颜色以适应背景色 */
    border-radius: 8px;
}

.el-tree-node__content:hover {
    background-color: #40a0ff33 !important; /* 这里设置为你需要的颜色 */
    border-radius: 8px;
}
</style>
