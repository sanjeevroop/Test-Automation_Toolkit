import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.waitForPageLoad(2)

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Policy Manager OSP Scheme Details/span_Your Absence Rules'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Policy Manager OSP Scheme Details/a_OSP Schemes'))

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Policy Manager OSP Scheme Service Bands/a_Service band tab'))

WebUI.delay(2)

//verify text
WebUI.verifyTextPresent('Select Scheme to maintain ', false)

WebUI.closeBrowser()

