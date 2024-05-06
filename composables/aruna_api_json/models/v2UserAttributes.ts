/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { v2CustomAttribute } from './v2CustomAttribute';
import type { v2DataProxyAttribute } from './v2DataProxyAttribute';
import type { v2OidcMapping } from './v2OidcMapping';
import type { v2Permission } from './v2Permission';
import type { v2Token } from './v2Token';
export type v2UserAttributes = {
    globalAdmin?: boolean;
    serviceAccount?: boolean;
    tokens?: Array<v2Token>;
    trustedEndpoints?: Array<string>;
    customAttributes?: Array<v2CustomAttribute>;
    personalPermissions?: Array<v2Permission>;
    externalIds?: Array<v2OidcMapping>;
    pubkey?: string;
    dataProxyAttributes?: Array<v2DataProxyAttribute>;
};

