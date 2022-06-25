import {useEffect, useRef} from "react";
import { invoke } from '@tauri-apps/api/tauri'

const Graph = props => {
  const canvasRef = useRef(null)

  useEffect(() => {
    const canvas = canvasRef.current
    const context = canvas.getContext('2d')

    invoke('generate_graph');
    console.log(canvasRef);


    //Our first draw
    context.fillStyle = '#000000'
    context.fillRect(0, 0, context.canvas.width, context.canvas.height)
  }, [])

  return <canvas className="w-full h-full" ref={canvasRef} {...props} />
}

export default Graph