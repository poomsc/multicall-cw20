import { create_wallet, execute, init, query, upload } from "./utils";
require("dotenv").config();
const wallet = create_wallet(process.env.DEVELOP_MODE === "true" ? process.env.MNEMONIC : process.env.PRODUCTION_MNEMONIC);

(async () => {
  const id = await upload(
    wallet,
    "../artifacts/multicall_cw20-aarch64.wasm"
  );
  const initRes = await init(wallet, id, {});
  console.log(id);
  console.log(initRes);

  const contractAddr = initRes.contract_addr;
  // const contractAddr = "terra1rpc4kp8rdl0hplqtgnlcjk5dre7uu3eqyz7z2e";
  const bulkBalance = await query(contractAddr, {
    bulk_balance: {
      contracts: [
        {
          contract_addr: "terra1747mad58h0w4y589y3sk84r5efqdev9q4r02pc",
          address: "terra1d35w0ad9gpav5dz9auz3p7zyazyt695q367afh",
        },
        {
          contract_addr: "terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl",
          address: "terra1d35w0ad9gpav5dz9auz3p7zyazyt695q367afh",
        },
      ]
    },
  });
  console.log(bulkBalance);
  const bulkTokenInfo = await query(contractAddr, {
    bulk_token_info: {
      contracts: [
        {
          contract_addr: "terra1747mad58h0w4y589y3sk84r5efqdev9q4r02pc",
        },
        {
          contract_addr: "terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl",
        },
      ]
    },
  });
  console.log(bulkTokenInfo);
})();
