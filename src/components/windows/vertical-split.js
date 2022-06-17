import Split from 'react-split'
import {renderToStaticMarkup} from "react-dom/server";

export default function VerticalSplit({ children, gutterClassName, gutterInnerElements }) {
  return (
      <Split
          direction="vertical"
          gutter={
            (index, direction) => {
              const output = document.createElement('div');
              output.innerHTML = (
                  renderToStaticMarkup(
                      <div className={`gutter gutter-${direction} ${gutterClassName}`}>
                        { gutterInnerElements }
                      </div>
                  )
              );
              return output;
            }
          }
          gutterStyle={
            (dimension, gutterSize, index) => {
              return (
                  {
                    'height': 'auto'
                  }
              )
            }
          }
          minSize={[0, 14]}
          className="bg-primary-200 h-full"
      >
        { children }
      </Split>
  )
}