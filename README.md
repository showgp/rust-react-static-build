
wasm 的内容简单梳理, 主要是如何将 wasm module 同 react 等前端框架结合使用的问题, 这里有一个 Rust + React 的例子: https://www.tkat0.dev/posts/how-to-create-a-react-app-with-rust-and-wasm/

1. 创建目录
1. 创建 react 工程: `npx create-react-app . --template typescript`
1. 使用 yarn 安装依赖: `yarn`
1. 启动 App 验证正确: `yarn start`
1. 在相同目录创建一个 rust 工程: `cargo new wasm-lib --lib`
1. 创建一个示例的代码(在 React 侧调用的), 这里使用生成 lib 时自带的 `add` 来测试, 因此不用修改代码, 仅演示流程.
1. 使用 `wasm-bindgen` 将该函数导出给 React 侧使用, 因此需要修改一些位置:
    - 添加 wasm-bindgen 依赖: `cargo add wasm-bindgen`
    - 修改 lib 的类型为 `cdylib`: 

        ```toml
        [lib]
        crate-type = ["cdylib"]
        ```

    - 使用 `#[wasm_bingen]` 注解将该函数标记为需要导出给 JS 的:

        ```rust
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn add(left: usize, right: usize) -> usize {
            left + right
        }
        ```

    - 使用 `wasm-pack` 构建 Wasm module:(如果没有安装, 可以使用 `cargo install wasm-pack` 安装), 这里在 `package.json` 中添加一个 build:

        ```
        "build:wasm": "cd wasm-lib && wasm-pack build --target web --out-dir ../pkg",
        ```
    
    - 执行这个构建看是否成功: `yarn build:wasm`
    - 同时因为 wasm-pack 帮我们在 pkg 目录下生成了 package.json, 因此在 react app 这边可以直接使用 `yarn add ./pkg` 就能依赖它, 且执行后可以发现 `package.json` 文件中多了一行 `"wasm-lib": "./pkg"`.
1. 下一步就是在 React 中调用导出的函数, 这个函数是在 wasm 中的, 因此有如下修改:
    - 引入导出的 `add` 函数以及一些帮助方法: `import init, { add } from "wasm-lib";`
    - 修改添加一点代码(调用 init 以及调用 add), 然后在界面上渲染 ans 更新的结果.

        ```tsx
        function App() {
          const [ans, setAns] = useState(0);

          useEffect(() => {
            init().then(() => {
                setAns(add(1, 2));
            })
          }, []);

          //...
          <p>To answer 1 + 2 = {ans}</p>
          // ...
        }
        ```

        上述代码的工作流程是: 当 App 这个 functional component 渲染完成后就调用 init, init 在做 wasm 的加载和初始化, 完成后再调用 `setAns`(`useEffect`), 由于 ans 是一个状态, 当 `setAns` 调用后, 对应位置的视图也会被更新(绑定展示状态到界面上).

1. 通过上面的步骤, 就完成了一套将核心业务逻辑放到 rust wasm 中, 且将展示及展示逻辑处理放到前端框架的整个流程.

上面的完成后, 后续需要调查的:

1. 如何静态导出? 参考这个: https://siddharthac6.medium.com/getting-started-with-react-js-using-webpack-and-babel-66549f8fbcb8
    - 示例工程中有 yarn build, 执行后有 build 目录.
    - 将 build 目录直接拖到 CF 的 Pages 中, 部署非常方便!
        - `yarn build`
        - 找到 build 目录, 压缩后拖到 CF 部署为 Pages 即可.
1. 如何对应不同的构建配置(debug, release)?
1. 使用 web-sys 的尝试: 引入是可以的. 每次修改了 rust 侧后, 需要调用一次 `yarn add ./pkg`.
1. 引入 wee 内存分配器
1. 如何部署?
1. 使用 `npm init rust-webpack` 这套便捷的模板是没有添加任何框架的, 但正常也可以用同一套逻辑处理应.