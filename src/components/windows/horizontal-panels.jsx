import { forwardRef, useImperativeHandle, useState, useRef } from 'react';
import useWindowSize from '../hooks/window-size';



const HorizontalPanels = forwardRef(( props, ref ) => {
    // props
    let [ leftElement, setLeftElement ] = useState(props.leftElement);
    let [ rightElement, setRightElement ] = useState(props.rightElement);
    const leftElementContainerStyle = props.leftElementContainerStyle;
    const rightElementContainerStyle = props.rightElementContainerStyle;
    const initialState = props.initialState;
    const gutterSize = props.gutterSize ? props.gutterSize : 20;
    const inititialSizes = props.inititialSizes ? props.inititialSizes : ['50%', '50%'];
    const fixedSizePanel = props.fixedSizePanel ? props.fixedSizePanel : 'left';
    let snap = props.snap ? props.snap : 40;

    // Local
    let [dragging, setDragging] = useState(false);

    let parent = useRef();
    let leftPanel = useRef();
    let rightPanel = useRef();
    let gutter = useRef();

    // Handles what panels to display
    const [ currentWindow, setCurrentWindow ] = useState(initialState);

    // Exposed functions
    useImperativeHandle(ref, () => ({

      closeWindow: (target) => {
        if (target == 'left') {
          if (currentWindow == 'both') {
            setCurrentWindow('right');
          } else if (currentWindow == 'left') {
            setCurrentWindow('none');
          } else {
            console.log("Trying to close a window already closed!");
          }
        } else if (target == 'right') {
          if (currentWindow == 'both') {
            setCurrentWindow('left');
          } else if (currentWindow == 'right') {
            setCurrentWindow('none');
          } else {
            console.log("Trying to close a window already closed!");
          }
        } else if (target == 'both') {
            setCurrentWindow('none');
        }
         else {
          console.log("Invalid data provided. Can only be 'left', 'right' or 'both'!")
        }
      },
  
      openWindow: (target) => {
        if (target == 'left') {
          if (currentWindow == 'right') {
            setCurrentWindow('both');
          } else if (currentWindow == 'none') {
            setCurrentWindow('left');
          } else {
            console.log("Trying to open an already open window!");
          }
        } else if (target == 'right') {
          if (currentWindow == 'left') {
            setCurrentWindow('both');
          } else if (currentWindow == 'none') {
            setCurrentWindow('right');
          } else {
            console.log("Trying to open an already open window!");
          }
        } else if (target == 'both') {
          setCurrentWindow('both');
        } else {
          console.log("Invalid data provided. Can only be 'left', 'right' or 'both'!")
        }
        
      },
  
      setWindowContent: (target, element) => {
        if (target == 'left') {
          setLeftElement(element);
        } else if (target == 'right') {
          setRightElement(element);
        } else {
          console.log("Invalid target provided, can only be 'left' or 'right'!");
        }
      },
  
      currentWindow: () => {
        return currentWindow;
      }
  
    }));

    // Global window size
    const windowSize = useWindowSize();

    function checkSizes() {
      if (!(leftPanel.current != undefined
        && rightPanel.current != undefined
        && leftPanelSize != undefined
        && rightPanelSize != undefined)) return;
      // Rerender if resizing
      let parentSize = parent.current.offsetWidth;
      if (leftPanelSize + rightPanelSize != parentSize) {
        // Keep the panelMinimized if it is needed
        let leftPanelMinized = leftPanelSize == 0;
        let rightPanelMinimized = rightPanelSize == 0;

        if (leftPanelMinized) {
          setRightPanelSize(parentSize);
        } else if (rightPanelMinimized) {
          setLeftPanelSize(parentSize);
        } else if (fixedSizePanel == 'left') {

          let newRightPanelSize = Math.max(0, parentSize - leftPanelSize);
          let newLeftPanelSize = Math.min(leftPanelSize, parentSize - newRightPanelSize);
          setLeftPanelSize(newLeftPanelSize);
          setRightPanelSize(newRightPanelSize);

        } else {
          if (fixedSizePanel != 'right') {
            console.log("Invalid fixedSizePanel value!")
          }

          let newLeftPanelSize = Math.max(0, parentSize - rightPanelSize);
          let newRightPanelSize = Math.min(rightPanelSize, parentSize - newLeftPanelSize);
          setLeftPanelSize(newLeftPanelSize);
          setRightPanelSize(newRightPanelSize);
        }

      }
    }

    // Panels size
    const [ leftPanelSize, setLeftPanelSize ] = useState(undefined);
    const [ rightPanelSize, setRightPanelSize ] = useState(undefined);


    function setPanelsWidth() {
      if (!(leftPanel.current != undefined
        && rightPanel.current != undefined
        && leftPanelSize != undefined
        && rightPanelSize != undefined)) return;   
      leftPanel.current.style.width = leftPanelSize + 'px';
      rightPanel.current.style.width = rightPanelSize + 'px';
    }
    
    function resize(leftSize) {
      let parentSize = parent.current.offsetWidth;

      let newSize = leftSize <= snap ? 0 : (parentSize - leftSize <= snap ? parentSize : leftSize); 

      setLeftPanelSize(newSize);
      setRightPanelSize(parentSize - newSize);
    }

    checkSizes();
    setPanelsWidth();
  
    return (
      <div
        className='w-full h-full relative overflow-hidden flex'
        ref={ el => { parent.current = el } }

        onMouseMove={ e => {
          if (currentWindow != 'both') return;
 
          // Left and right absolute positions of the window container
          let leftAbs = parent.current.getBoundingClientRect().x;
          let rightAbs = parent.current.getBoundingClientRect().right;

          // Check if is inside limits
          if (
            dragging && 
            e.pageX >= leftAbs + gutterSize/2 &&
            e.pageX <= rightAbs - gutterSize/2 
          ) resize(e.pageX - leftAbs);
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
          // Left panel
          (currentWindow == 'both' || currentWindow == 'left') &&
          <div
            className={`h-full overflow-hidden flex-none ${leftElementContainerStyle}`}
            ref={el => { leftPanel.current = el }}
            style={
              {'width' : (
                currentWindow == 'both' ?
                (leftPanelSize == undefined ? inititialSizes[0] : leftPanelSize + 'px')
                : '100%'
              )}
            }
          >
            { leftElement }
          </div>
        }
        {
          // Right panel
          (currentWindow == 'both' || currentWindow == 'right') &&
          <div
            className={`h-full overflow-hidden flex-none ${rightElementContainerStyle}`}
            ref={el => { rightPanel.current = el }}
            style={
              {'width' : (
                currentWindow == 'both' ?
                (rightPanelSize == undefined ? inititialSizes[1] : rightPanelSize + 'px')
                : '100%' 
              )}
            }
          >
            { rightElement }
          </div>
        }

        {
          // Gutter
          currentWindow == 'both' &&
          <div
            className='absolute bg-transparent h-full cursor-w-resize group flex justify-center'
            ref={el => { gutter.current = el }}
            style={
              {
                'width': `${gutterSize}px`,
                'transform': `translateX(-${gutterSize/2}px)`,
                'left': (leftPanelSize == undefined ? inititialSizes[0] : leftPanelSize + 'px')
              }
            }
  
            onMouseDown={(e) => {
              e.preventDefault();
              setDragging(true);
              parent.current.style.cursor = 'w-resize';
            }}
          >
            <div
              className={`h-full w-px transition-all duration-75 delay-100 bg-neutral-600
              ${dragging ? 'w-1/6 bg-amber-500' : 'group-hover:w-1/6 group-hover:bg-amber-500'}`}
            />
          </div>
        }

      </div>
    )
})

export default HorizontalPanels;