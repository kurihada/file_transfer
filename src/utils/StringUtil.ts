const _ = {
    isNullAndEmpty(obj: any): boolean {
        return typeof obj === 'undefined' || obj === null || obj === ''
    },
    isNotNullAndEmpty(obj: any): boolean {
        return typeof obj !== 'undefined' && obj !== null && obj !== ''
    },
}

export default _
