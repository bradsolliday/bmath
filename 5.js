(window.webpackJsonp=window.webpackJsonp||[]).push([[5],{2:function(t,e,n){"use strict";n.r(e);var a=n(4),r=n(8);function o(t,e){for(var n=0;n<e.length;n++){var a=e[n];a.enumerable=a.enumerable||!1,a.configurable=!0,"value"in a&&(a.writable=!0),Object.defineProperty(t,a.key,a)}}var i=function(){function t(e,n,a,r,o){var i=this;!function(t,e){if(!(t instanceof e))throw new TypeError("Cannot call a class as a function")}(this,t),a.addEventListener("click",(function(t){i.toggleListener(t,a)})),a.height=5*e.rows(),a.width=5*e.cols(),this.data=e,this.typeArray=o,this.ctx=a.getContext("2d"),this.memory=n,this.colorMap=r,this.animationId=null,this.renderLoop=function(){i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.data.update(),i.drawCells(),i.animationId=requestAnimationFrame(i.renderLoop)}}var e,n,a;return e=t,(n=[{key:"getIndex",value:function(t,e){return t*this.data.rows()+e}},{key:"drawCells",value:function(){var t=this.data.rows(),e=this.data.cols(),n=new this.typeArray(this.memory.buffer,this.data.data_pointer(),t*e),a=this.ctx;a.beginPath();for(var r=0;r<t;r++)for(var o=0;o<e;o++){var i=this.getIndex(r,o);a.fillStyle=this.colorMap(n[i]),a.fillRect(5*o,5*(t-1-r),5,5)}a.stroke()}},{key:"isPaused",value:function(){return null===this.animationId}},{key:"play",value:function(){this.renderLoop()}},{key:"pause",value:function(){cancelAnimationFrame(this.animationId),this.animationId=null}},{key:"render",value:function(){this.drawCells()}},{key:"toggleListener",value:function(t,e){var n=e.getBoundingClientRect(),a=e.width/n.width,r=e.height/n.height,o=(t.clientX-n.left)*a,i=(t.clientY-n.top)*r,u=this.data.rows(),l=this.data.cols(),c=l-1-Math.max(Math.min(Math.floor(i/5),l-1),0),s=Math.max(Math.min(Math.floor(o/5),u-1),0);this.data.toggle_cell(c,s),this.drawCells()}},{key:"reset",value:function(){this.data.reset(),this.drawCells()}}])&&o(e.prototype,n),a&&o(e,a),t}(),u=n(5),l=n.n(u),c=n(6),s=n.n(c);function f(t){return(f="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(t){return typeof t}:function(t){return t&&"function"==typeof Symbol&&t.constructor===Symbol&&t!==Symbol.prototype?"symbol":typeof t})(t)}function d(t,e){for(var n=0;n<e.length;n++){var a=e[n];a.enumerable=a.enumerable||!1,a.configurable=!0,"value"in a&&(a.writable=!0),Object.defineProperty(t,a.key,a)}}function p(t,e){return(p=Object.setPrototypeOf||function(t,e){return t.__proto__=e,t})(t,e)}function h(t){var e=function(){if("undefined"==typeof Reflect||!Reflect.construct)return!1;if(Reflect.construct.sham)return!1;if("function"==typeof Proxy)return!0;try{return Date.prototype.toString.call(Reflect.construct(Date,[],(function(){}))),!0}catch(t){return!1}}();return function(){var n,a=m(t);if(e){var r=m(this).constructor;n=Reflect.construct(a,arguments,r)}else n=a.apply(this,arguments);return y(this,n)}}function y(t,e){return!e||"object"!==f(e)&&"function"!=typeof e?function(t){if(void 0===t)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return t}(t):e}function m(t){return(m=Object.setPrototypeOf?Object.getPrototypeOf:function(t){return t.__proto__||Object.getPrototypeOf(t)})(t)}for(var v=function(t){!function(t,e){if("function"!=typeof e&&null!==e)throw new TypeError("Super expression must either be null or a function");t.prototype=Object.create(e&&e.prototype,{constructor:{value:t,writable:!0,configurable:!0}}),e&&p(t,e)}(o,t);var e,n,a,r=h(o);function o(t){var e;return function(t,e){if(!(t instanceof e))throw new TypeError("Cannot call a class as a function")}(this,o),(e=r.call(this,t)).state={text:"▶"},e}return e=o,(n=[{key:"render",value:function(){var t=this;return l.a.createElement("button",{onClick:function(){t.props.isPaused()?(t.props.play(),t.setState({text:"⏸"})):(t.props.pause(),t.setState({text:"▶"}))}},this.state.text)}}])&&d(e.prototype,n),a&&d(e,a),o}(l.a.Component),b=[],w=0;w<256;w++){var g=w.toString(16);1===g.length&&(g="0"+g),b[w]="#"+g+g+g}var k=document.getElementById("my-canvas"),x=new i(a.c.new(),r.m,k,(function(t){return t<0&&alert("pixel value was negative"),t>255&&alert("pixel value was greater than 255"),b[Math.floor(t)]}),Float32Array);s.a.render(l.a.createElement(l.a.Fragment,null,l.a.createElement(v,{isPaused:x.isPaused.bind(x),play:x.play.bind(x),pause:x.pause.bind(x)}),l.a.createElement("button",{onClick:x.reset.bind(x)},"Reset Field")),document.getElementById("animation-button")),x.render()}}]);