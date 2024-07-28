export interface FileInfo {
    name: string;
    path: string;
    is_file: boolean;
    is_dir: boolean;
    parent_id: string | number;
    children?: FileInfo[];
}
