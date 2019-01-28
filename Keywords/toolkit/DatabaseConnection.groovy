package toolkit
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords

import internal.GlobalVariable

import MobileBuiltInKeywords as Mobile
import WSBuiltInKeywords as WS
import WebUiBuiltInKeywords as WebUI

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException

import java.sql.DriverManager

import java.sql.ResultSet

import com.mysql.jdbc.Connection
import com.mysql.jdbc.Statement

public class DemoMySql {

private static Connection connection = null

/**

* Open and return a connection to database

* @param dataFile absolute file path

* @return an instance of java.sql.Connection

*/

//Establishing a connection to the DataBase

@Keyword

def connectDB(String url, String dbname, String port, String username, String password){

//Load driver class for your specific database type

String conn = "jdbc:mysql://" + url + ":" + port + "/" + dbname

//Class.forName("org.sqlite.JDBC")

//String connectionString = "jdbc:sqlite:" + dataFile

if(connection != null && !connection.isClosed()){

connection.close()

}

connection = DriverManager.getConnection(conn, username, password)

return connection

}

/**

* execute a SQL query on database

* @param queryString SQL query string

* @return a reference to returned data collection, an instance of java.sql.ResultSet

*/

//Executing the constructed Query and Saving results in resultset

@Keyword

def executeQuery(String queryString) {

Statement stm = connection.createStatement()

ResultSet rs = stm.executeQuery(queryString)

return rs

}

//Closing the connection

@Keyword

def closeDatabaseConnection() {

if(connection != null && !connection.isClosed()){

connection.close()

}

connection = null

}

/**

* Execute non-query (usually INSERT/UPDATE/DELETE/COUNT/SUM...) on database

* @param queryString a SQL statement

* @return single value result of SQL statement

*/

@Keyword

def execute(String queryString) {

Statement stm = connection.createStatement()

boolean result = stm.execute(queryString)

return result

}

}
class DatabaseConnection {
	/**
	 * Refresh browser
	 */
	@Keyword
	def refreshBrowser() {
		KeywordUtil.logInfo("Refreshing")
		WebDriver webDriver = DriverFactory.getWebDriver()
		webDriver.navigate().refresh()
		KeywordUtil.markPassed("Refresh successfully")
	}

	/**
	 * Click element
	 * @param to Katalon test object
	 */
	@Keyword
	def clickElement(TestObject to) {
		try {
			WebElement element = WebUiBuiltInKeywords.findWebElement(to)
			KeywordUtil.logInfo("Clicking element")
			element.click()
			KeywordUtil.markPassed("Element has been clicked")
		} catch (WebElementNotFoundException e) {
			KeywordUtil.markFailed("Element not found")
		} catch (Exception e) {
			KeywordUtil.markFailed("Fail to click on element")
		}
	}

	/**
	 * Get all rows of HTML table
	 * @param table Katalon test object represent for HTML table
	 * @param outerTagName outer tag name of TR tag, usually is TBODY
	 * @return All rows inside HTML table
	 */
	@Keyword
	def List<WebElement> getHtmlTableRows(TestObject table, String outerTagName) {
		WebElement mailList = WebUiBuiltInKeywords.findWebElement(table)
		List<WebElement> selectedRows = mailList.findElements(By.xpath("./" + outerTagName + "/tr"))
		return selectedRows
	}
}