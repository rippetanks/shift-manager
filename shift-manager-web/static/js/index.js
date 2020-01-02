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

$(document).ready(function() {
  if(SessionStorage.getToken() == null) {
    window.location.href = './login';
  }
  API.loadShiftStructure().done(function(structure, message, e) {
    // se non esiste la struttura del turno rimando alla pagina impostazioni per la creazione
    if(e.status == 204) {
      alert('Non hai ancora creato la struttura del tuo turno! Creala adesso.');
      window.location.href = './settings';
    }
    API.loadShiftExpansion(structure.id).done(function(expansion) {
      CalendarAPI.init(structure, expansion);
    }).fail(function(e) {
      API.logError(e);
    });
  }).fail(function(e) {
    if(e.status == 401) {
      window.location.href = './login';
    }
  });

  $('#datetimepicker').datetimepicker({
    locale: 'it',
    format: 'YYYY-MM',
    viewMode: 'months'
  });
  $('#datetimepicker').on('change.datetimepicker', function(date) {
    let newDate = date.date.format('YYYY-MM-DD');
    CalendarAPI.goToDate(newDate);
  });

  $('#exit').click(function(e) {
    SessionStorage.setToken(null);
    window.location.href = './login';
  });
});
