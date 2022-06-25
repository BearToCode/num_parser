import { useRef } from "react";
import Graph from "../components/graphs/graph";
import HorizontalPanels from "../components/windows/horizontal-panels";

export default function TwoDimensions() {
  const horizontalPanelsRef = useRef();


  return (
      <div className="h-full w-full">
        <HorizontalPanels 
          ref={horizontalPanelsRef} 
          leftElement={
            <div className="">
              Hi
            </div>
          }
          rightElement={<div>mom!</div>}
          leftElementContainerStyle={'bg-primary-200'}
          rightElementContainerStyle={'bg-primary-200'}
          initialState={'both'}
        />
      </div>
  );
}
