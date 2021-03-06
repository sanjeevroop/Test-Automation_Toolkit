import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW

WebUI.waitForPageLoad(10)

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Statutory Absence Parameters/span_Your Absence Rules'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Statutory Absence Parameters/a_Statutory Absence'))

WebUI.delay(2)

WebUI.click(findTestObject('Navigations/Navigation - Statutory Absence Notifications/a_Statutory Absence Notificati'))

WebUI.delay(2)

WebUI.verifyElementPresent(findTestObject('Navigations/Navigation - Statutory Absence Notifications/button_Save'), 0)

//verify page title in div
WebUI.verifyTextPresent('Statutory Absence Notifications', false, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.scrollToElement(findTestObject('Navigations/Navigation - Statutory Absence Notifications/button_Save'), 5)

WebUI.click(findTestObject('Navigations/Navigation - Statutory Absence Notifications/button_Save'))

WebUI.verifyTextPresent('Statutory Absence Notifications', false, FailureHandling.CONTINUE_ON_FAILURE)

