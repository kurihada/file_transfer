import { ElNotification } from 'element-plus';

let _ = {
    error(description: string) {
        ElNotification({
            type: 'error',
            title: '通知',
            message: description,
            position: 'bottom-right',
            duration: 3200,
        });
    },

    success(description: string) {
        ElNotification({
            type: 'success',
            title: '通知',
            message: description,
            position: 'bottom-right',
            duration: 3200,
        });
    },

    warning(description: string) {
        ElNotification({
            type: 'warning',
            title: '通知',
            message: description,
            position: 'bottom-right',
            duration: 3200,
        });
    },
};

export default _;
