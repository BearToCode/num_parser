import Split from 'react-split'
import {renderToStaticMarkup} from "react-dom/server";

export default function VerticalSplit({ children }) {
  return (
      <Split
          direction="vertical"
          gutter={
            (index, direction) => {
              const output = document.createElement('div');
              output.innerHTML = (
                  renderToStaticMarkup(
                      <div
                        className={`gutter gutter-${direction} bg-transparent h-full group cursor-row-resize`}
                      >
                        <div className="h-px group-hover:h-full transform -translate-y-1/2 duration-200
                        bg-neutral-600 group-hover:bg-amber-500">

                        </div>

                      </div>
                  )
              );
              return output;
            }
          }
          gutterSize={4}
          minSize={[0, 24]}
          className="bg-primary-200 h-full"
      >
        { children }
      </Split>
  )
}