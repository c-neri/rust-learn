"use client";
import axios from "axios";
import { ChangeEvent, useState } from "react";

export default function Home() {
  const [files, setFiles] = useState<FileList | null>(null);
  const [ids, setIds] = useState<string[]>([]);

  const submit = async () => {
    if (!files) return;
    const formData = new FormData();
    for (let i = 0; i < files.length; i++) {
      formData.append("upload", files[i]);
    }
    try {
      const res = await axios({
        method: "post",
        url: "http://127.0.0.1:8080/upload",
        data: formData,

        headers: { "Content-Type": "multipart/form-data" },
      });
      res.data && setIds(res.data.ids);
      setFiles(null);
    } catch (error) {
      console.log(error);
    }
  };
  const style =
    "bg-gray-900 m-auto w-72 border p-2 rounded-lg text-gray-100 hover:cursor-pointer";
  return (
    <div className="container h-screen m-auto flex  items-center justify-center">
      <div className="shadow-lg  flex flex-col gap-4 border bg-gray-950 rounded-lg p-4">
        <h1 className="text-2xl text-gray-100">Rust File Uploader</h1>
        <input
          className={style}
          multiple
          type="file"
          onChange={(e) => setFiles(e.target.files)}
        ></input>
        <button className={style} onClick={submit}>
          Upload
        </button>
        {ids && ids.length > 0 && (
          <div className="mt-4 flex flex-col gap-2">
            {ids.map((id) => {
              return (
                <div
                  className="text-xs text-gray-50 p-4 bg-gray-800 rounded-lg"
                  key={id}
                >
                  {id}
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
