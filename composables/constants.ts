
export const PROJECT_REGEX: RegExp = new RegExp('^[a-z0-9\-]+$')
export const S3_KEY_REGEX: RegExp = new RegExp('^[a-zA-Z0-9\-\!\_\.\*\_\'\(\)]+$')
export const OBJECT_REGEX: RegExp = new RegExp('^[a-zA-Z0-9\-\!\_\.\*\_\'\(\)\/]+$')
export const ULID_REGEX: RegExp = new RegExp('^[0-7][0-9A-HJKMNP-TV-Z]{25}$')
export const ORCID_REGEX: RegExp = new RegExp('(\\d{4}-){3}\\d{3}(\\d|X)$')
export const EMAIL_REGEX: RegExp= new RegExp('^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\\.[a-zA-Z0-9-]+)*$')