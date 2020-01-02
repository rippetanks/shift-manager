/*
  Copyright (C) 2019  Simone Martelli

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU Affero General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU Affero General Public License for more details.

  You should have received a copy of the GNU Affero General Public License
  along with this program. If not, see <https://www.gnu.org/licenses/>.

  Contact info:
  - email:  simone.martelli.98@gmail.com
*/

var app = null;

$(document).ready(function() {

  app = new Vue({
    el: "#app",
    data: {
      email: null,
      pwd: null
    },
    methods: {
      login: function() {
        Login.login(this.email, this.pwd);
      },
      signup: function() {
        Login.signup(this.email, this.pwd);
      }
    }
  });

  $("#signup").click(function() {
    $("#first").fadeOut("fast", function() {
      $("#second").fadeIn("fast");
    });
  });

  $("#signin").click(function() {
    $("#second").fadeOut("fast", function() {
      $("#first").fadeIn("fast");
    });
  });

});

/**
* Login App
*/
const Login = (function() {

  function login(email, pwd) {
    if(validateLogin(email, pwd)) {
      let hash = getHash(pwd);
      API.login(email, hash).done(function(data) {
        SessionStorage.setToken(data.token);
        window.location.href = '../';
      }).fail(function(e) {
        API.logError(e);
      });
    } else {
      console.log('Login validation failed!');
    }
  }

  function signup(email, pwd) {
    if(validateSignup(email, pwd)) {
      let hash = getHash(pwd);
      API.signup(email, hash).done(function() {
        alert('Account creato con successo!');
        window.location.reload();
      }).fail(function(e) {
        API.logError(e);
      });
    } else {
      console.log('Signup validation failed!');
    }
  }

  function validateLogin(email, password) {
    return validateEmail(email, app.$refs.loginEmail) && validatePassword(password, app.$refs.loginPwd);
  }

  function validateSignup(email, password) {
    return validateEmail(email, app.$refs.signupEmail) && validatePassword(password, app.$refs.signupPwd);
  }

  function validateEmail(email, el) {
    let filter = /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/;
    if(email == null || email === "" || !filter.test(email)) {
      console.error("Email is not valid!");
      $(el).addClass('error');
      return false;
    } else {
      $(el).removeClass('error');
    }
    return true;
  }

  function validatePassword(password, el) {
    if(password == null || password === "") {
      console.error("Password is not valid!");
      $(el).addClass('error');
      return false;
    } else {
      $(el).removeClass('error');
    }
    return true;
  }

  function getHash(text) {
    return sha3_512(text);
  }

  // public
  return {
    login: login,
    signup: signup
  };

})();
