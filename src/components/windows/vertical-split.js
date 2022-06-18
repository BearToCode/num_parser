import Split from 'react-split'
import {renderToStaticMarkup} from "react-dom/server";

export default function VerticalSplit({ children, gutterClassName }) {
  return (
      <Split
          direction="vertical"
          gutter={
            (index, direction) => {
              const output = document.createElement('div');
              output.innerHTML = (
                  renderToStaticMarkup(
                      <div className={`gutter gutter-${direction} ${gutterClassName}`}>

                      </div>
                  )
              );
              return output;
            }
          }
          className="bg-primary-200 h-full"
      >
        { children }
      </Split>
  )
}