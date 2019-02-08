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

//WebUI.openBrowser('https://implementationtoolkit.azurewebsites.net/')
//WebUI.navigateToUrl('https://implementationtoolkit.azurewebsites.net/')
//WebUI.maximizeWindow()
//WebUI.setText(findTestObject('Navigations/Navigation - Pay Groups/input_loginfmt'), 'Sanjeev@sdworx.com')
//WebUI.click(findTestObject('Navigations/Navigation - Pay Groups/input_idSIButton9'))
WebUI.click(findTestObject('Navigations/Navigation - Pay Groups/a_Your Payroll Rules'))

WebUI.waitForElementPresent(findTestObject('Navigations/Navigation - Pay Groups/a_Pay Groups'), 0, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Navigations/Navigation - Pay Groups/a_Pay Groups'))

WebUI.verifyElementPresent(findTestObject('Navigations/Navigation - Pay Groups/a_Add Paygroup'), 0)

