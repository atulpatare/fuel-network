/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import type { Provider, BaseWalletLocked, AbstractAddress } from "fuels";
import { Interface, Contract } from "fuels";
import type { CounterAbi, CounterAbiInterface } from "../CounterAbi";
const _abi = [
  {
    type: "function",
    name: "count",
    inputs: [],
    outputs: [
      {
        type: "u64",
        name: "",
      },
    ],
  },
  {
    type: "function",
    name: "increment",
    inputs: [],
    outputs: [
      {
        type: "()",
        name: "",
        components: [],
      },
    ],
  },
  {
    type: "function",
    name: "increment_custom",
    inputs: [
      {
        type: "u64",
        name: "value",
      },
    ],
    outputs: [
      {
        type: "()",
        name: "",
        components: [],
      },
    ],
  },
  {
    type: "function",
    name: "decrement",
    inputs: [],
    outputs: [
      {
        type: "()",
        name: "",
        components: [],
      },
    ],
  },
  {
    type: "function",
    name: "decrement_custom",
    inputs: [
      {
        type: "u64",
        name: "value",
      },
    ],
    outputs: [
      {
        type: "()",
        name: "",
        components: [],
      },
    ],
  },
];

export class CounterAbi__factory {
  static readonly abi = _abi;
  static createInterface(): CounterAbiInterface {
    return new Interface(_abi) as unknown as CounterAbiInterface;
  }
  static connect(
    id: string | AbstractAddress,
    walletOrProvider: BaseWalletLocked | Provider
  ): CounterAbi {
    return new Contract(id, _abi, walletOrProvider) as unknown as CounterAbi;
  }
}
