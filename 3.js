(window.webpackJsonp=window.webpackJsonp||[]).push([[3],{3:function(t,e,n){"use strict";n.r(e);var r=n(5),a=n.n(r),o=n(6),u=n.n(o),i=n(4);function c(t){return(c="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(t){return typeof t}:function(t){return t&&"function"==typeof Symbol&&t.constructor===Symbol&&t!==Symbol.prototype?"symbol":typeof t})(t)}function l(t,e){for(var n=0;n<e.length;n++){var r=e[n];r.enumerable=r.enumerable||!1,r.configurable=!0,"value"in r&&(r.writable=!0),Object.defineProperty(t,r.key,r)}}function s(t,e){return(s=Object.setPrototypeOf||function(t,e){return t.__proto__=e,t})(t,e)}function p(t){var e=function(){if("undefined"==typeof Reflect||!Reflect.construct)return!1;if(Reflect.construct.sham)return!1;if("function"==typeof Proxy)return!0;try{return Date.prototype.toString.call(Reflect.construct(Date,[],(function(){}))),!0}catch(t){return!1}}();return function(){var n,r=m(t);if(e){var a=m(this).constructor;n=Reflect.construct(r,arguments,a)}else n=r.apply(this,arguments);return f(this,n)}}function f(t,e){return!e||"object"!==c(e)&&"function"!=typeof e?h(t):e}function h(t){if(void 0===t)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return t}function m(t){return(m=Object.setPrototypeOf?Object.getPrototypeOf:function(t){return t.__proto__||Object.getPrototypeOf(t)})(t)}var y=function(t){!function(t,e){if("function"!=typeof e&&null!==e)throw new TypeError("Super expression must either be null or a function");t.prototype=Object.create(e&&e.prototype,{constructor:{value:t,writable:!0,configurable:!0}}),e&&s(t,e)}(u,t);var e,n,r,o=p(u);function u(t){var e;return function(t,e){if(!(t instanceof e))throw new TypeError("Cannot call a class as a function")}(this,u),(e=o.call(this,t)).state={input1:1,input2:1,arg1:1,arg2:1,output:t.op(1,1),dtime:0},e.handleChange=e.handleChange.bind(h(e)),e.calculate=e.calculate.bind(h(e)),e}return e=u,(n=[{key:"handleChange",value:function(t){var e=t.target.name,n=Math.floor(Number(t.target.value));n<=0&&(n=1);var r=this.props.maxInput;n>r&&(n=r),this.setState(function(t,e,n){return e in t?Object.defineProperty(t,e,{value:n,enumerable:!0,configurable:!0,writable:!0}):t[e]=n,t}({},e,n));var a={input1:this.state.input1,input2:this.state.input2};a[e]=n,this.calculate(a.input1,a.input2)}},{key:"calculate",value:function(t,e){Date.now();var n=this.props.op(t,e);Date.now(),this.setState({arg1:t,arg2:e,output:n})}},{key:"render",value:function(){return a.a.createElement(a.a.Fragment,null,a.a.createElement("input",{name:"input1",type:"number",step:"1",value:this.state.input1,onChange:this.handleChange}),a.a.createElement("input",{name:"input2",type:"number",step:"1",value:this.state.input2,onChange:this.handleChange}),a.a.createElement("p",null,this.props.outputMessage(this.state.arg1,this.state.arg2,this.state.output)),a.a.createElement("p",null,"(Time to calculate: ",this.state.dtime," miliseconds)"))}}])&&l(e.prototype,n),r&&l(e,r),u}(a.a.Component);u.a.render(a.a.createElement(a.a.Fragment,null,a.a.createElement("div",{className:"demo"},a.a.createElement(y,{op:i.i,outputMessage:function(t,e,n){return"The gcd of "+t+" and "+e+" is "+n+"."},maxInput:4294967295})),a.a.createElement("div",{className:"demo"},a.a.createElement(y,{op:i.j,outputMessage:function(t,e,n){var r=" + ",a=n.b();return a<0&&(r=" - ",a=-a),n.a()+"*"+t+r+a+"*"+e+" = "+n.gcd()},maxInput:2147483647}))),document.getElementById("gcd-demo"))}}]);