export interface Response<T> {
    code: string;
    message: string;
    data: T;
}

export const NOT_FOUND_CODE = 'A0001';
