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

/**
*
*/
const API = (function() {

  let baseURL = 'https://rippetanks.ddns.net:3000';
  //let baseURL = 'http://localhost:8000';

  function http_get(url, cb) {
    return $.ajax({
      url: url,
      dataType: 'json',
      type: 'GET',
      contentType: 'application/json',
      headers: {
        "Authentication": SessionStorage.getToken()
      },
      success: function(data, textStatus, jQxhr) {
        if(cb)
          cb(data);
      },
      error: function(jQxhr, textStatus, errorThrown) {
        console.error(errorThrown);
      }
    });
  }

  function http_post(url, data, cb) {
    return $.ajax({
      url: url,
      dataType: 'json',
      type: 'POST',
      contentType: 'application/json',
      data: JSON.stringify(data),
      headers: {
        "Authentication": SessionStorage.getToken()
      },
      success: function(data, textStatus, jQxhr){
        if(cb)
          cb(data);
      },
      error: function(jQxhr, textStatus, errorThrown) {
        console.error(errorThrown);
      }
    });
  }

  function http_put(url, data, cb) {
    return $.ajax({
      url: url,
      dataType: 'json',
      type: 'PUT',
      contentType: 'application/json',
      data: JSON.stringify(data),
      headers: {
        "Authentication": SessionStorage.getToken()
      },
      success: function(data, textStatus, jQxhr){
        if(cb)
          cb(data);
      },
      error: function(jQxhr, textStatus, errorThrown) {
        console.error(errorThrown);
      }
    });
  }

  function http_delete(url, cb) {
    return $.ajax({
      url: url,
      type: 'DELETE',
      headers: {
        "Authentication": SessionStorage.getToken()
      },
      success: function(data, textStatus, jQxhr){
        if(cb)
          cb(data);
      },
      error: function(jQxhr, textStatus, errorThrown) {
        console.error(errorThrown);
      }
    });
  }

  /*
  function keepalive() {
    setTimeout(function() {
      http_get(baseURL + '/keepalive').done(function(token) {
        SessionStorage.setToken(data.token);
      }).fail(function(e) {
        console.error("KEEPALIVE FAILED!");
        console.log(e);
      });
    }, 60000);
  }
  */

  // public
  return {
    login: function(email, password, cb) {
      return http_post(baseURL + '/user/login', {email: email, password: password}, cb);
    },
    signup: function(email, password, cb) {
      return http_put(baseURL + '/user', {email: email, password: password}, cb);
    },
    loadShiftStructure: function(cb) {
      return http_get(baseURL + '/shift/structure/user', cb);
    },
    loadShiftExpansion: function(id, cb) {
      return http_get(baseURL + '/shift/expansion/structure/' + id, cb);
    },
    updateShiftStructure: function(structure, cb) {
      return http_post(baseURL + '/shift/structure/' + structure.id, structure, cb);
    },
    updateShiftExpansion: function(id, expansion, cb) {
      return http_post(baseURL + '/shift/expansion/' + id, expansion, cb);
    },
    insertShiftStructure: function(structure, cb) {
      return http_put(baseURL + '/shift/structure', structure, cb);
    },
    insertShiftExpansion: function(id, expansion, cb) {
      return http_put(baseURL + '/shift/expansion/' + id, expansion, cb);
    },
    deleteShiftStructure: function(id, cb) {
      return http_delete(baseURL + '/shift/structure' + id, cb);
    },
    deleteShiftExpansion: function(id, cb) {
      return http_delete(baseURL + '/shift/expansion/' + id, cb);
    },

    logError: function(e) {
      console.log(e);
      alert('HTTP ' + e.status + ' (' + e.statusText + ')! ' + e.responseText);
    }
  };

})();
