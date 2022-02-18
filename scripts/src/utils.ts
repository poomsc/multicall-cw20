import {
  MsgStoreCode,
  StdFee,
  LCDClient,
  MnemonicKey,
  MsgInstantiateContract,
  Coins,
  MsgExecuteContract,
  Wallet,
  MsgSend,
  Coin,
  MsgMigrateContract,
  StdTx,
} from "@terra-money/terra.js";
import * as fs from "fs";
const mnemonicWords = require("mnemonic-words");

const DELAY_TIME = 1000; // this to prevent unauthorization error
const GAS_LIMIT = 10000000;

const networks = {
  localterra: {
    URL: "http://localhost:1317",
    chainID: "localterra",
    gasAdjustment: 1.5,
  },
  testnet: {
    URL: "https://bombay-lcd.terra.dev",
    chainID: "bombay-12",
    gasAdjustment: 1.5,
  },
  mainnet: {
    URL: "https://lcd.terra.dev",
    chainID: "columbus-5",
    gasAdjustment: 1.5,
  },
};

const terra = new LCDClient(networks["testnet"]);

export const instantiate = async (network) => {
  return new LCDClient(networks[network]);
};

export const create_wallet = (mnemonic) => {
  const key = new MnemonicKey({
    mnemonic: mnemonic,
  });
  return terra.wallet(key);
};

export const upload = async (
  wallet,
  path,
  fee = "5000000uusd"
): Promise<Number> => {
  const tx = await wallet.createAndSignTx({
    msgs: [
      new MsgStoreCode(
        wallet.key.accAddress,
        fs.readFileSync(path, { encoding: "base64" })
      ),
    ],
    fee: new StdFee(GAS_LIMIT, fee),
  });
  try {
    const response = await terra.tx.broadcast(tx);
    console.log(`upload response for file ${path}`);
    await delay(DELAY_TIME);
    console.log("RAW LOG", response.raw_log);
    const logs = JSON.parse(response.raw_log);
    let code_id = "";
    logs.forEach((log) => {
      log.events.forEach((event) => {
        if (event.type == "store_code") {
          code_id = event.attributes.find(
            (attribute) => attribute.key == "code_id"
          ).value;
        }
      });
    });
    await delay(DELAY_TIME);
    return Number(code_id);
  } catch (err) {
    // console.log("err ", err);
    throw err;
  }
};

export const init = async (wallet, code_id, init_msg, fee = "5000000uusd") => {
  const tx = await wallet.createAndSignTx({
    msgs: [
      new MsgInstantiateContract(
        wallet.key.accAddress,
        wallet.key.accAddress,
        code_id,
        init_msg
      ),
    ],
    fee: new StdFee(GAS_LIMIT, fee),
  });
  const response = await terra.tx.broadcast(tx);
  await delay(DELAY_TIME);
  try {
    console.log("init response ", response);
    const logs = JSON.parse(response.raw_log);
    let contract_addr = "";
    logs.forEach((log) => {
      log.events.forEach((event) => {
        if (event.type == "instantiate_contract") {
          contract_addr = event.attributes.find(
            (attribute) => attribute.key == "contract_address"
          ).value;
        }
      });
    });
    return {
      contract_addr: contract_addr,
      logs,
    };
  } catch (err) {
    console.log("err ", err, response);
    throw err;
  }
};

export const execute = async (
  wallet: Wallet,
  addr,
  execute_msg,
  coins?,
  fee = "2000000uusd"
) => {
  let coin = new Coins(coins);
  // if (coins) coin = new Coins(coins);
  const tx: StdTx = await wallet.createAndSignTx({
    msgs: [
      new MsgExecuteContract(wallet.key.accAddress, addr, execute_msg, coin),
    ],
    fee: new StdFee(GAS_LIMIT, fee),
  });
  const response = await terra.tx.broadcast(tx);
  await delay(DELAY_TIME);
  return response;
};

export const executes = async (
  wallet: Wallet,
  addr,
  execute_msgs: Array<any>,
  coins?,
  fee = "2000000uusd"
) => {
  let coin = new Coins(coins);
  const tx: StdTx = await wallet.createAndSignTx({
    msgs: execute_msgs.map(
      (execute_msg) =>
        new MsgExecuteContract(wallet.key.accAddress, addr, execute_msg, coin)
    ),
    fee: new StdFee(GAS_LIMIT, fee),
  });
  const response = await terra.tx.broadcast(tx);
  await delay(DELAY_TIME);
  return response;
};

export const execute_bulk = async (
  wallet: Wallet,
  addr,
  msgs,
  coins?,
  fee = "1500000uusd"
) => {
  let coin = new Coins();
  if (coins) coin = Coins.fromString(coins);
  const tx: StdTx = await wallet.createAndSignTx({
    msgs: msgs,
    fee: new StdFee(GAS_LIMIT, fee),
  });
  const response = await terra.tx.broadcast(tx);
  await delay(DELAY_TIME);
  return response;
};

export const migrate = async (
  wallet,
  addr,
  code_id,
  migrate_msg,
  fee = "5000000uusd"
) => {
  const tx = await wallet.createAndSignTx({
    msgs: [
      new MsgMigrateContract(wallet.key.accAddress, addr, code_id, migrate_msg),
    ],
    fee: new StdFee(GAS_LIMIT, fee),
  });
  try {
    const response = await terra.tx.broadcast(tx);
    await delay(DELAY_TIME);
    return response;
  } catch (err) {
    throw err;
  }
};

export const signature = async (
  wallet: Wallet,
  addr,
  execute_msg,
  coins?,
  fee = "1500000uusd"
) => {
  let coin = new Coins();
  if (coins) coin = Coins.fromString(coins);
  const tx: StdTx = await wallet.createAndSignTx({
    msgs: [
      new MsgExecuteContract(wallet.key.accAddress, addr, execute_msg, coin),
    ],
    fee: new StdFee(GAS_LIMIT, fee),
  });
  const msg = "hello world";
  const sign = await wallet.key.sign(Buffer.from(msg));
  console.log(sign.toString("base64"));
  let tx2 = await wallet.createTx({
    msgs: [
      new MsgExecuteContract(wallet.key.accAddress, addr, execute_msg, coin),
    ],
    fee: new StdFee(GAS_LIMIT, fee),
  });
  console.log(tx2);
  return {
    msg: Buffer.from(tx2.toJSON()).toString("base64"),
    signature: (await wallet.key.sign(Buffer.from(tx2.toJSON()))).toString(
      "base64"
    ),
    pub_key: wallet.key.publicKey.toString("base64"),
  };
};

export const transfer = async (
  wallet: Wallet,
  addr,
  coins,
  fee = "2500000uusd"
) => {
  const tx = await wallet.createAndSignTx({
    msgs: [new MsgSend(wallet.key.accAddress, addr, Coins.fromString(coins))],
    fee: new StdFee(GAS_LIMIT, fee),
  });
  const response = await terra.tx.broadcast(tx);
  await delay(DELAY_TIME);
  return response;
};

export const balance = async (addr) => {
  return await terra.bank.balance(addr);
};

export const query = async (addr, msg) => {
  const response = await terra.wasm.contractQuery(addr, msg);
  return response;
};

export const delay = (ms) => {
  return new Promise((resolve) => setTimeout(resolve, ms, {}));
};

export const randomMnemonic = () => {
  return Array.from({ length: 24 }, () => Math.floor(Math.random() * 2047)).map(
    (i) => mnemonicWords[i]
  );
};
