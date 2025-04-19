const HomePage = () => {
  const onSend = async () => {
   // let res = await fetch("http://localhost:3030/room/create", { method: "POST" });
  };

  return (
    <div className="">
      <button onClick={onSend}>send</button>
    </div>
  );
};

export { HomePage };
