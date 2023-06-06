const WebSocket = require("ws");

const ws = new WebSocket("ws://localhost:3000/ws");

ws.on("open", function () {
  console.log("连接已打开");
  let msg = {
    room: "room1",
    username: "jack",
    timestamp: Date.now(),
    data: {
      message: "hello",
    },
  };
  let data = JSON.stringify(msg);
  console.log("send data: " + data);
  ws.send(data);
});

ws.on("message", function (data) {
  console.log("收到消息：" + data + "\n");
});

ws.on("close", function () {
  console.log("连接已关闭");
});
