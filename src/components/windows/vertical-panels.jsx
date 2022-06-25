import { forwardRef, useImperativeHandle, useState, useRef } from 'react';
import useWindowSize from '../hooks/window-size';



const VerticalPanels = forwardRef(( props, ref ) => {
    // props
    let [ topElement, setTopElement ] = useState(props.topElement);
    let [ bottomElement, setBottomElement ] = useState(props.bottomElement);
    const topElementContainerStyle = props.topElementContainerStyle;
    const bottomElementContainerStyle = props.bottomElementContainerStyle;
    const initialState = props.initialState;
    const gutterSize = props.gutterSize ? props.gutterSize : 20;
    const inititialSizes = props.inititialSizes ? props.inititialSizes : ['50%', '50%'];
    const fixedSizePanel = props.fixedSizePanel ? props.fixedSizePanel : 'top';
    let snap = props.snap ? props.snap : 40;

    // Local
    let [dragging, setDragging] = useState(false);

    let parent = useRef();
    let topPanel = useRef();
    let bottomPanel = useRef();
    let gutter = useRef();

    // Handles what panels to display
    const [ currentWindow, setCurrentWindow ] = useState(initialState);

    // Exposed functions
    useImperativeHandle(ref, () => ({

      closeWindow: (target) => {
        if (target == 'top') {
          if (currentWindow == 'both') {
            setCurrentWindow('bottom');
          } else if (currentWindow == 'top') {
            setCurrentWindow('none');
          } else {
            console.log("Trying to close a window already closed!");
          }
        } else if (target == 'bottom') {
          if (currentWindow == 'both') {
            setCurrentWindow('top');
          } else if (currentWindow == 'bottom') {
            setCurrentWindow('none');
          } else {
            console.log("Trying to close a window already closed!");
          }
        } else if (target == 'both') {
            setCurrentWindow('none');
        }
         else {
          console.log("Invalid data provided. Can only be 'top', 'bottom' or 'both'!")
        }
      },
  
      openWindow: (target) => {
        if (target == 'top') {
          if (currentWindow == 'bottom') {
            setCurrentWindow('both');
          } else if (currentWindow == 'none') {
            setCurrentWindow('top');
          } else {
            console.log("Trying to open an already open window!");
          }
        } else if (target == 'bottom') {
          if (currentWindow == 'top') {
            setCurrentWindow('both');
          } else if (currentWindow == 'none') {
            setCurrentWindow('bottom');
          } else {
            console.log("Trying to open an already open window!");
          }
        } else if (target == 'both') {
          setCurrentWindow('both');
        } else {
          console.log("Invalid data provided. Can only be 'top', 'bottom' or 'both'!")
        }
        
      },
  
      setWindowContent: (target, element) => {
        if (target == 'top') {
          setTopElement(element);
        } else if (target == 'bottom') {
          setBottomElement(element);
        } else {
          console.log("Invalid target provided, can only be 'top' or 'bottom'!");
        }
      },
  
      currentWindow: () => {
        return currentWindow;
      }
  
    }));

    // Global window size
    const windowSize = useWindowSize();

    function checkSizes() {
      if (!(topPanel.current != undefined
        && bottomPanel.current != undefined
        && topPanelSize != undefined
        && bottomPanelSize != undefined)) return;
      // Rerender if resizing
      let parentSize = parent.current.offsetHeight;
      if (topPanelSize + bottomPanelSize != parentSize) {
        // Keep the panelMinimized if it is needed
        let topPanelMinimized = topPanelSize == 0;
        let bottomPanelMinimized = bottomPanelSize == 0;

        if (topPanelMinimized) {
          setBottomPanelSize(parentSize);
        } else if (bottomPanelMinimized) {
          setTopElement(parentSize);
        } else if (fixedSizePanel == 'top') {

          let newBottomPanelSize = Math.max(0, parentSize - topPanelSize);
          let newTopPanelSize = Math.min(topPanelSize, parentSize - newBottomPanelSize);
          setTopPanelSize(newTopPanelSize);
          setBottomPanelSize(newBottomPanelSize);

        } else {
          if (fixedSizePanel != 'bottom') {
            console.log("Invalid fixedSizePanel value!")
          }

          let newTopPanelSize = Math.max(0, parentSize - bottomPanelSize);
          let newBottomPanelSize = Math.min(bottomPanelSize, parentSize - newTopPanelSize);
          setTopPanelSize(newTopPanelSize);
          setBottomPanelSize(newBottomPanelSize);

        }
        
      }
    }

    // Panels size
    const [ topPanelSize, setTopPanelSize ] = useState(undefined);
    const [ bottomPanelSize, setBottomPanelSize ] = useState(undefined);


    function setPanelsWidth() {
      if (!(topPanel.current != undefined
        && bottomPanel.current != undefined
        && topPanelSize != undefined
        && bottomPanelSize != undefined)) return;   
      topPanel.current.style.height = topPanelSize + 'px';
      bottomPanel.current.style.height = bottomPanelSize + 'px';
    }
    
    function resize(leftSize) {
      let parentSize = parent.current.offsetHeight;

      let newSize = leftSize <= snap ? 0 : (parentSize - leftSize <= snap ? parentSize : leftSize); 

      setTopPanelSize(newSize);
      setBottomPanelSize(parentSize - newSize);
    }

    checkSizes();
    setPanelsWidth();
  
    return (
      <div
        className='w-full h-full relative overflow-hidden flex flex-col'
        ref={ el => { parent.current = el } }

        onMouseMove={ e => {
          if (currentWindow != 'both') return;
 
          // Left and right absolute positions of the window container
          let topAbs = parent.current.getBoundingClientRect().y;
          let bottomAbs = parent.current.getBoundingClientRect().bottom;

          // Check if is inside limits
          if (
            dragging && 
            e.pageY >= topAbs + gutterSize/2 &&
            e.pageY <= bottomAbs - gutterSize/2 
          ) resize(e.pageY - topAbs);
        } }

        // Handle leaving stop dragging
        onMouseLeave={(e) => {
          if (currentWindow != 'both') return;

          setDragging(false);
          parent.current.style.cursor = 'default';
        }}

        onMouseUp={(e) => {
          if (currentWindow != 'both') return;

          setDragging(false);
          parent.current.style.cursor = 'default';
        }}
      >

        {
          // Top panel
          (currentWindow == 'both' || currentWindow == 'top') &&
          <div
            className={`w-full overflow-hidden flex-none ${topElementContainerStyle}`}
            ref={el => { topPanel.current = el }}
            style={
              {'height' : (
                currentWindow == 'both' ?
                (topPanelSize == undefined ? inititialSizes[0] : topPanelSize + 'px')
                : '100%'
              )}
            }
          >
            { topElement }
          </div>
        }
        {
          // Bottom panel
          (currentWindow == 'both' || currentWindow == 'bottom') &&
          <div
            className={`w-full overflow-hidden flex-none ${bottomElementContainerStyle}`}
            ref={el => { bottomPanel.current = el }}
            style={
              {'height' : (
                currentWindow == 'both' ?
                (bottomPanelSize == undefined ? inititialSizes[1] : bottomPanelSize + 'px')
                : '100%' 
              )}
            }
          >
            { bottomElement }
          </div>
        }

        {
          // Gutter
          currentWindow == 'both' &&
          <div
            className='absolute bg-transparent w-full cursor-n-resize group flex items-center'
            ref={el => { gutter.current = el }}
            style={
              {
                'height': `${gutterSize}px`,
                'transform': `translateY(-${gutterSize/2}px)`,
                'top': (topPanelSize == undefined ? inititialSizes[0] : topPanelSize + 'px')
              }
            }
  
            onMouseDown={(e) => {
              e.preventDefault();
              setDragging(true);
              parent.current.style.cursor = 'n-resize';
            }}
          >
            <div
              className={`w-full h-px transition-all duration-75 delay-100 bg-neutral-600
              ${dragging ? 'h-1/6 bg-amber-500' : 'group-hover:h-1/6 group-hover:bg-amber-500'}`}
            />
          </div>
        }

      </div>
    )
})

export default VerticalPanels;