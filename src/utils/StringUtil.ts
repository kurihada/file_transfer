import NotificationUtil from './NotificationUtil';

const _ = {
    isNullAndEmpty(obj: any): boolean {
        return typeof obj === 'undefined' || obj === null || obj === '';
    },
    isNotNullAndEmpty(obj: any): boolean {
        return typeof obj !== 'undefined' && obj !== null && obj !== '';
    },
    namingVerify(name: string): boolean {
        if (/.*[/\\:*?|].*/.test(name)) {
            NotificationUtil.error('文件夹命名不规范，不能包含\\ / : * ? " < > |请检查！');
            return false;
        }
        if (/^\d+$/.test(name)) {
            NotificationUtil.error('文件夹命名不规范，不能为纯数字！');
            return false;
        }
        return true;
    },
};

export default _;
