
import { Sha256 } from "https://deno.land/std@0.119.0/hash/sha256.ts";
import { Hex } from "./hex.ts";
import { HookParameter, HookGrant, HookPayload } from "./types/hooks.ts";

export class SimplifiedHooksToolkit {
  static hexNamespace(hookNamespaceSeed: string): string {
    return new Sha256().update(hookNamespaceSeed).hex().toUpperCase()
  }

  static hexHookParameters(data: HookParameter[]): HookParameter[] {
    const hookParameters: HookParameter[] = []
    for (const parameter of data) {
      let hookPName = parameter.HookParameter.HookParameterName
      let hookPValue = parameter.HookParameter.HookParameterValue
  
      if (!Hex.isHex(hookPName)) {
        hookPName = Hex.stringToHexString(hookPName)
      }
  
      if (!Hex.isHex(hookPValue)) {
        hookPValue = Hex.stringToHexString(hookPValue)
      }
  
      hookParameters.push({
        HookParameter: {
          HookParameterName: hookPName,
          HookParameterValue: hookPValue,
        },
      })
    }
    return hookParameters
  }

  static createHookPayload(
    version?: number | null,
    namespace?: string | null,
    flags?: number | 0,
    hookOn = `0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffbfffff`,
    hookParams?: HookParameter[] | null,
    hookGrants?: HookGrant[] | null
  ): HookPayload {
    const hook = {
      hookOn 
    } as HookPayload
    if (typeof version === 'number') {
      hook.HookApiVersion = version
    }
    if (namespace) {
      hook.HookNamespace = this.hexNamespace(namespace)
    }
    if (flags) {
      hook.Flags = flags
    }
    if (hookParams) {
      hook.HookParameters = this.hexHookParameters(hookParams)
    }
    if (hookGrants) {
      hook.HookGrants = hookGrants
    }
    return hook
  }
}