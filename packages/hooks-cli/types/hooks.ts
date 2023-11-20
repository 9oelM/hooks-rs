export interface HookParameter {
  /**
   * The object that describes the parameter in HookParameters.
   */
  HookParameter: {
    /**
     * The name of the parameter.
     */
    HookParameterName: string;
    /**
     * The value of the parameter.
     */
    HookParameterValue: string;
  };
}

/**
 * The object that describes the grant in HookGrants.
 */
export interface HookGrant {
  /**
   * The object that describes the grant in HookGrants.
   */
  HookGrant: {
    /**
     * The hook hash of the grant.
     */
    HookHash: string;
    /**
     * The account authorized on the grant.
     */
    Authorize?: string;
  };
}

export type HookPayload = {
  // HookHash?: string
  CreateCode?: string;
  Flags?: number;
  HookOn?: string;
  HookNamespace?: string;
  HookApiVersion?: number;
  HookParameters?: HookParameter[];
  HookGrants?: HookGrant[];
};
